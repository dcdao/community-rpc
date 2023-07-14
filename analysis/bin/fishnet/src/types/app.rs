use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub struct FishnetStartInput {
  /// Envoy admin endpoint
  #[structopt(short, long, default_value = "http://127.0.0.1:9001")]
  pub endpoint: String,
}
