use crate::error::FishnetResult;
use crate::store::cluster_store::ClusterStore;
use crate::types::app::FishnetStartInput;
use crate::types::envoy::ClusterInfo;

pub struct EnvoyAnalysisService {
  input: FishnetStartInput,
}

impl EnvoyAnalysisService {
  pub fn new(input: FishnetStartInput) -> Self {
    Self { input }
  }
}

impl EnvoyAnalysisService {
  pub async fn analysis(&self) -> FishnetResult<()> {
    let cluster_info = reqwest::get(format!("{}/clusters?format=json", self.input.endpoint))
      .await?
      .json::<ClusterInfo>()
      .await?;
    // println!("{:?}", cluster_info);
    let data_path = self.input.safe_data_path()?;
    let store = ClusterStore::new(data_path);

    let mut stored_state = store.current_state().await?;
    for cluster_status in cluster_info.cluster_statuses {
      for host_status in cluster_status.host_statuses {
        // tracing::debug!(
        //   target: "fishnet",
        //   "handle cluster info {} -> {}",
        //   cluster_status.name,
        //   host_status.endpoint()
        // );
        stored_state
          .increase(cluster_status.name.clone(), host_status)
          .await?;
      }
    }
    Ok(())
  }
}
