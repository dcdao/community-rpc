use crate::service::envoy_analysis::EnvoyAnalysisService;
use crate::types::app::FishnetStartInput;

pub async fn handle_start(input: FishnetStartInput) -> color_eyre::Result<()> {
  tracing::info!(target: "fishnet", "start fishnet");
  // let bridge_config: RawBridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
  // init_bridge(bridge, bridge_config).await?;
  let service = EnvoyAnalysisService::new(input);
  loop {
    if let Err(e) = service.analysis().await {
      tracing::error!(target: "fishnet", "failed to analysis envoy status log: {:?}", e);
    }
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
  }
}
