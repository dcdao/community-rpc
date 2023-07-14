use crate::error::FishnetResult;
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
    println!("{:?}", cluster_info);
    Ok(())
  }
}
