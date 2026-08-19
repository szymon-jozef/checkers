#[cfg(test)]
mod tests {

    use tokio::time::Duration;

    use checkers::{
        network::server::Server,
        super_advanced_ai::{Bot, BotDificulty},
    };

    #[tokio::test]
    async fn game_integration() {
        let _ = env_logger::builder().is_test(true).try_init();

        tokio::spawn(async move {
            let mut server = Server::new().await;
            server.start().await;
            server.update().await;
        });

        tokio::time::sleep(Duration::from_secs(2)).await;

        let bot1_handle = tokio::spawn(async move {
            let bot1 = Bot::new(BotDificulty::Easy).await;
            bot1.game_loop().await
        });

        let bot2_handle = tokio::spawn(async move {
            let bot2 = Bot::new(BotDificulty::Easy).await;
            bot2.game_loop().await
        });

        let bot1_result = bot1_handle.await.unwrap();
        let bot2_result = bot2_handle.await.unwrap();

        if bot1_result.is_some() && bot2_result.is_some() {
            assert_eq!(bot1_result.unwrap(), bot2_result.unwrap());
        } else if bot1_result.is_none() && bot2_result.is_none() {
            ()
        } else {
            panic!("Bot result differanciate");
        }
    }
}
