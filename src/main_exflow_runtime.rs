mod mod_azure;

#[tokio::main]
async fn main() {
  mod_azure::azure::test().await
}
