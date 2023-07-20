use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterInfo {
  pub cluster_statuses: Vec<ClusterStatus>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterStatus {
  pub name: String,
  pub added_via_api: bool,
  pub circuit_breakers: CircuitBreakers,
  pub host_statuses: Vec<HostStatus>,
  pub observability_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HostStatus {
  pub address: HostStateAddress,
  pub stats: Vec<HStat>,
  pub health_status: HealthStatus,
  pub weight: u32,
  pub hostname: String,
}

impl HostStatus {
  pub fn id(&self, cluster: impl AsRef<str>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(cluster.as_ref());
    hasher.update(self.endpoint());
    let hash_result = hasher.finalize();
    format!("{:x}", hash_result)
  }
  pub fn endpoint(&self) -> String {
    format!(
      "{}:{}",
      self.address.socket_address.address, self.address.socket_address.port_value
    )
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HostStateAddress {
  pub socket_address: SocketAddress,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SocketAddress {
  pub address: String,
  pub port_value: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HStat {
  pub name: String,
  pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HealthStatus {
  pub eds_health_status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CircuitBreakers {
  pub thresholds: Vec<CBThreshold>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CBThreshold {
  pub max_connections: u64,
  pub max_pending_requests: u64,
  pub max_requests: u64,
  pub max_retries: u64,
}

pub struct HStatPlus {
  stats: Vec<HStat>,
}

impl HStatPlus {
  pub fn new(stats: Vec<HStat>) -> Self {
    Self { stats }
  }
}

impl HStatPlus {
  pub fn value(&self, name: impl AsRef<str>) -> Option<String> {
    for stat in &self.stats {
      if stat.name == name.as_ref() {
        return Some(stat.value.clone());
      }
    }
    None
  }

  pub fn value_int(&self, name: impl AsRef<str>) -> Option<u64> {
    let value = self.value(name)?;
    u64::from_str(&value).ok()
  }
}
