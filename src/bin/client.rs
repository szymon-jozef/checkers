use std::io;

use checkers::network::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let Some(mut client) = Client::new().await else {
        println!("Not worky, ending");
        return Ok(());
    };

    client.update();

    println!("=== Write to stding to send text messages ===");
    println!("Write /close to leave");
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        if buffer == "/close\n" {
            break;
        }

        if buffer == "/test\n" {
            buffer = String::from("A").repeat(1024 * 1024 * 2);
        }

        client.send_text_message(buffer.clone()).await;
        buffer.clear();
    }

    return Ok(());
}
