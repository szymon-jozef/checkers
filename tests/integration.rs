#[cfg(test)]
mod tests {
    use tokio::time::Duration;

    use checkers::{
        network::server::Server,
        super_advanced_ai::{Bot, BotDificulty},
    };

    #[tokio::test]
    async fn game_integration() {
        tokio::spawn(async move {
            let mut server = Server::new().await;
            server.start().await;
            server.update().await;
        });

        tokio::time::sleep(Duration::from_secs(2)).await; // wait for the server to run

        let bot1_handle = tokio::spawn(async move {
            let bot1 = Bot::new(BotDificulty::Easy).await;
            bot1.game_loop().await;
        });

        let bot2_handle = tokio::spawn(async move {
            let bot2 = Bot::new(BotDificulty::Easy).await;
            bot2.game_loop().await;
        });

        bot1_handle.await.unwrap();
        bot2_handle.await.unwrap();
    }
}
