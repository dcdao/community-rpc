use crate::error::InitializerError;

mod init_log;

pub mod error;

pub fn init() -> Result<(), InitializerError> {
  init_log::init_log()?;
  Ok(())
}
