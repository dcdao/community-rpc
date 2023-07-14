use crate::handler;
use crate::types::command::Opts;

/// Execute command
pub async fn execute(opts: Opts) -> color_eyre::Result<()> {
  match opts {
    Opts::Start { input } => handler::handle_start(input).await,
  }
}
