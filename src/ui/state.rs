use std::sync::mpsc::{self, Receiver, Sender};

use crate::{
    network::{client::Client, server::Server},
    super_advanced_ai::{Bot, BotDificulty},
    ui::macroquad::game::game::GameClient,
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

#[derive(Clone)]
pub struct GameContext {
    pub difficulty: BotDificulty,
    pub gamemode: GameMode,
    pub server_url_buffer: String,
}

impl Default for GameContext {
    fn default() -> Self {
        Self {
            difficulty: BotDificulty::default(),
            gamemode: GameMode::default(),
            server_url_buffer: "127.0.0.1".to_string(),
        }
    }
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

pub fn run_server(sender: Sender<()>) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let mut server = rt.block_on(Server::new());
        let _ = rt.block_on(server.start());

        let _ = sender.send(());

        rt.block_on(server.update());
    });
}

pub fn connect_bot(dificluty: BotDificulty) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let bot = rt.block_on(Bot::new(dificluty));

        rt.block_on(bot.game_loop());
    });
}
