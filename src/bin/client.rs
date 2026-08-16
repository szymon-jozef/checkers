use checkers::network::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut client: Client = Client::new().await;
    client.update().await;

    return Ok(());
}
