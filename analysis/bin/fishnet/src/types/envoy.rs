use serde::{Deserialize, Serialize};

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
