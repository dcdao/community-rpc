use structopt::StructOpt;

use crate::types::app::FishnetStartInput;

/// Fishnet operations
#[derive(Debug, StructOpt)]
#[structopt(name = "fishnet", about = "Community RPC Analysis Fishnet")]
pub enum Opts {
  /// Start fishnet
  Start {
    #[structopt(flatten)]
    input: FishnetStartInput,
  },
}
