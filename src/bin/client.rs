use std::io;

use checkers::network::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut client: Client = Client::new().await;
    client.update();

    println!("=== Write to stding to send text messages ===");
    println!("Write /close to leave");
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        if buffer == "/close\n" {
            break;
        }

        client.send_text_message(buffer.clone()).await;
        buffer.clear();
    }

    return Ok(());
}
