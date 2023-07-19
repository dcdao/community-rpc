use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterReport {
  pub id: String,
  pub name: String,
  pub endpoint: String,
  pub hostname: String,
  pub weight: u32,
  pub rq_error: u64,
  pub rq_success: u64,
  pub rq_timeout: u64,
  pub rq_total: u64,
}
