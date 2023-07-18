use std::path::PathBuf;

use tokio::fs;

use crate::error::FishnetResult;
use crate::types::envoy::{HStatPlus, HostStatus};
use crate::types::report::ClusterReport;

pub struct ClusterStore {
  data_path: PathBuf,
}

impl ClusterStore {
  pub fn new(data_path: PathBuf) -> Self {
    Self { data_path }
  }
}

impl ClusterStore {
  async fn safe_file_month(&self) -> FishnetResult<PathBuf> {
    self.safe_file_month_custom("").await
  }

  async fn safe_file_month_custom(&self, month: impl AsRef<str>) -> FishnetResult<PathBuf> {
    let mut month = month.as_ref().to_string();
    if month.is_empty() {
      month = format!("{}", chrono::Utc::now().format("%Y-%m"));
    }
    let dir = self.data_path.join(format!("cluster/{}", month));
    if !dir.exists() {
      fs::create_dir_all(&dir).await?;
    }
    Ok(dir.join("cluster.json"))
  }

  async fn safe_file_latest(&self) -> FishnetResult<PathBuf> {
    let dir = self.data_path.join("cluster");
    if !dir.exists() {
      fs::create_dir_all(&dir).await?;
    }
    Ok(dir.join("latest.json"))
  }
}

impl ClusterStore {
  pub async fn current_state(&self) -> FishnetResult<ClusterState> {
    let file_month = self.safe_file_month().await?;
    let file_latest = self.safe_file_latest().await?;
    if !file_month.exists() {
      return Ok(ClusterState {
        file_month,
        file_latest,
        reports: vec![],
      });
    }
    let content_month = fs::read_to_string(&file_month).await?;
    let reports = serde_json::from_str(&content_month).unwrap_or(vec![]);
    Ok(ClusterState {
      file_month,
      file_latest,
      reports,
    })
  }
}

pub struct ClusterState {
  file_month: PathBuf,
  file_latest: PathBuf,
  reports: Vec<ClusterReport>,
}

impl ClusterState {
  pub async fn increase(
    &mut self,
    cluster_name: String,
    host_status: HostStatus,
  ) -> FishnetResult<()> {
    let content_latest = fs::read_to_string(&self.file_latest).await?;
    let mut latests: Vec<ClusterReport> = serde_json::from_str(&content_latest).unwrap_or(vec![]);

    let hstat_plus = HStatPlus::new(host_status.stats.clone());
    let id = host_status.id(&cluster_name);

    let rq_error = hstat_plus.value_int("rq_error").unwrap_or(0);
    let rq_success = hstat_plus.value_int("rq_success").unwrap_or(0);
    let rq_timeout = hstat_plus.value_int("rq_timeout").unwrap_or(0);
    let rq_total = hstat_plus.value_int("rq_total").unwrap_or(0);

    // update month report
    match self.reports.iter_mut().find(|report| report.id == id) {
      None => {
        self.reports.push(ClusterReport {
          id: id.clone(),
          name: cluster_name.clone(),
          endpoint: host_status.endpoint(),
          hostname: host_status.hostname.clone(),
          weight: host_status.weight,
          rq_error,
          rq_success,
          rq_timeout,
          rq_total,
        });
      }
      Some(report) => match latests.iter().find(|&item| item.id == id) {
        None => {
          report.rq_error = rq_error;
          report.rq_success = rq_success;
          report.rq_timeout = rq_timeout;
          report.rq_total = rq_total;
        }
        Some(latest) => {
          report.rq_error += if rq_error < latest.rq_error {
            rq_error
          } else {
            rq_error - latest.rq_error
          };
          report.rq_success += if rq_success < latest.rq_success {
            rq_success
          } else {
            rq_success - latest.rq_success
          };
          report.rq_timeout += if rq_timeout < latest.rq_timeout {
            rq_timeout
          } else {
            rq_timeout - latest.rq_timeout
          };
          report.rq_total += if rq_total < latest.rq_total {
            rq_total
          } else {
            rq_total - latest.rq_total
          };
        }
      },
    }

    // update latest report
    match latests.iter_mut().find(|item| item.id == id) {
      None => {
        latests.push(ClusterReport {
          id,
          name: cluster_name.clone(),
          endpoint: host_status.endpoint(),
          hostname: host_status.hostname.clone(),
          weight: host_status.weight,
          rq_error,
          rq_success,
          rq_timeout,
          rq_total,
        });
      }
      Some(report) => {
        report.rq_error = rq_error;
        report.rq_success = rq_success;
        report.rq_timeout = rq_timeout;
        report.rq_total = rq_total;
      }
    };

    let json_month = serde_json::to_string_pretty(&self.reports)?;
    let json_latest = serde_json::to_string_pretty(&latests)?;
    fs::write(&self.file_month, &json_month).await?;
    fs::write(&self.file_latest, &json_latest).await?;
    tracing::info!(
      target: "fishnet",
      "write cluster report to {}",
      self.file_month.display()
    );
    Ok(())
  }
}
