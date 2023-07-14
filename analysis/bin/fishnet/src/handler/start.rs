use crate::service::envoy_analysis::EnvoyAnalysisService;
use crate::types::app::FishnetStartInput;

pub async fn handle_start(input: FishnetStartInput) -> color_eyre::Result<()> {
  tracing::info!(target: "fishnet", "start fishnet");
  // let bridge_config: RawBridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
  // init_bridge(bridge, bridge_config).await?;
  let service = EnvoyAnalysisService::new(input);
  service.analysis().await?;
  Ok(())
}
