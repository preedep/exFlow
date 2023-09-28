use log::{info};
mod mod_azure;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();

  info!("Test");
  mod_azure::azure::test().await
}
