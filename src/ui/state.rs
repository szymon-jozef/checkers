use std::sync::mpsc::{self, Receiver, Sender};

use crate::{
    network::client::Client, super_advanced_ai::BotDificulty, ui::macroquad::game::GameClient,
};

#[derive(Default)]
pub enum GuiState {
    #[default]
    MainMenu,

    ModeSelection,
    DificultySelection,
    ServerSelection,

    Connecting(Receiver<Option<Client>>),

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

enum CliCommands {
    Send,

    Ready,
    Unready,

    Capture,
    Move,
}

// TODO! Move this
pub fn connect_to_server() -> Receiver<Option<Client>> {
    let (tx, rc) = mpsc::channel::<Option<Client>>();

    let _ = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let client_option = rt.block_on(async { Client::new(None).await });

        let _ = tx.send(client_option);

        std::thread::park();
    });

    rc
}
