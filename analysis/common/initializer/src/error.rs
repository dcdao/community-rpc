use thiserror::Error as ThisError;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum InitializerError {
  #[error("Custom: {0}")]
  Custom(String),
}
