use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::error::{FishnetError, FishnetResult};

#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct FishnetStartInput {
  /// Envoy admin endpoint
  #[structopt(short, long, default_value = "http://127.0.0.1:9001")]
  pub endpoint: String,
  /// Store data path [default: <binary-path>/report]
  #[structopt(short, long)]
  pub data_path: Option<String>,
}

impl FishnetStartInput {
  pub fn safe_data_path(&self) -> FishnetResult<PathBuf> {
    if let Some(data_path) = &self.data_path {
      return Ok(Path::new(data_path).to_path_buf());
    }
    let data_path = std::env::current_exe()?
      .parent()
      .map(|item| item.join("report"))
      .ok_or_else(|| FishnetError::Custom(format!("cannot get running dir")))?;
    if !data_path.exists() {
      std::fs::create_dir_all(&data_path)?;
    }
    return Ok(data_path);
  }
}
