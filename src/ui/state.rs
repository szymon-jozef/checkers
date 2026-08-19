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
pub struct GameContext {
    pub difficulty: BotDificulty,
    pub is_single: bool,
    pub server_url_buffer: String,
}
