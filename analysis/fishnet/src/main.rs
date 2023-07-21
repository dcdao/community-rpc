use structopt::StructOpt;

use crate::types::command::Opts;

mod cli;
mod error;
mod handler;
mod initializer;
mod service;
mod store;
mod types;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  initializer::init()?;
  let opt = Opts::from_args();
  cli::execute(opt).await?;
  Ok(())
}
