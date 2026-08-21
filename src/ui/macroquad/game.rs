use tokio::sync::mpsc::Receiver;

use crate::{
    logic::board::board_view::BoardView,
    network::{client::Client, message::ServerMessage},
    ui::state::GameContext,
};

pub struct GameClient {
    client: Client,
    update_receiver: Receiver<ServerMessage>,

    board: BoardView,
    player_name: String,
    enemy_name: String, // TODO! Server should give this info to us (it doesn't rn)
}

impl GameClient {
    pub async fn new(context: &GameContext) -> Option<Self> {
        let Some(mut client) = Client::new(None).await else {
            return None;
        };

        let Some(update_receiver) = client.get_update_receiver() else {
            return None;
        };

        let board = BoardView::default();

        Some(Self {
            client,
            update_receiver,
            board,
            player_name: "Morbius".to_string(),
            enemy_name: "Milo".to_string(),
        })
    }
}
