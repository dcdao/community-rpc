use structopt::StructOpt;

use crate::types::command::Opts;

mod cli;
mod error;
mod handler;
mod service;
mod types;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  cmta_initializer::init()?;
  let opt = Opts::from_args();
  cli::execute(opt).await?;
  Ok(())
}
