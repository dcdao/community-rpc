use thiserror::Error as ThisError;

pub type FishnetResult<T> = Result<T, FishnetError>;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum FishnetError {
  #[error("Custom: {0}")]
  Custom(String),
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
}
