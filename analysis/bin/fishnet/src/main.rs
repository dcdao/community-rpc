mod types;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  cmta_initializer::init()?;
  println!("Hello, world!");
  tracing::debug!("Hello");
  Ok(())
}
