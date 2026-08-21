use std::sync::mpsc::{self, Receiver};

use crate::{super_advanced_ai::BotDificulty, ui::macroquad::game::GameClient};

#[derive(Default)]
pub enum GuiState {
    #[default]
    MainMenu,

    ModeSelection,
    DificultySelection,
    ServerSelection,

    Connecting(Receiver<Option<GameClient>>),

    Settings,
    Game(GameClient),
    Exit,
}

#[derive(Default, Clone, Copy)]
pub enum GameMode {
    #[default]
    Singleplayer,
    Multiplayer {
        is_hosting: bool,
    },
}

#[derive(Default, Clone)]
pub struct GameContext {
    pub difficulty: BotDificulty,
    pub gamemode: GameMode,
    pub server_url_buffer: String,
}

// TODO! Move this
pub fn connect_to_server(context: GameContext) -> Receiver<Option<GameClient>> {
    let (tx, rc) = mpsc::channel::<Option<GameClient>>();
    let context_clone = context.clone();

    let _ = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client_option = rt.block_on(async { GameClient::new(&context_clone).await });

        let _ = tx.send(client_option);

        std::thread::park();
    });

    rc
}
