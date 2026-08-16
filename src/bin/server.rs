use checkers::network::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server: Server = Server::new().await;

    env_logger::init();

    server.start().await;
    server.update().await;

    Ok(())
}
