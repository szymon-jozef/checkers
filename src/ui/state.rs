use crate::super_advanced_ai::BotDificulty;

#[derive(Default)]
pub enum GuiState {
    #[default]
    MainMenu,

    ModeSelection,
    DificultySelection,
    ServerSelection,

    Settings,
    Game,
    Exit,
}

#[derive(Default)]
pub enum GameMode {
    #[default]
    Singleplayer,
    Multiplayer {
        is_hosting: bool,
    },
}

#[derive(Default)]
pub struct GameContext {
    pub difficulty: BotDificulty,
    pub gamemode: GameMode,
    pub server_url_buffer: String,
}
