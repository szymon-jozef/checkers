use macroquad::{
    color::GRAY,
    math::vec2,
    shapes::draw_rectangle,
    ui::{hash, root_ui, widgets::Group},
    window::{next_frame, screen_height, screen_width},
};

use crate::ui::{
    macroquad::menu_builder::MenuBuilder,
    state::{
        GameContext,
        GuiState::{self, ServerSelection},
    },
};

pub async fn draw_mode_selection(state: &mut GuiState, context: &mut GameContext) {
    let menu_size = vec2(screen_width() * 0.5, screen_height() * 0.5);
    let menu_pos = vec2(
        screen_width() / 2.0 - menu_size.x / 2.0,
        screen_height() / 2.0 - menu_size.y / 2.0,
    );

    let mut menu_builder = MenuBuilder::new(menu_size.x, menu_size.y * 0.1);

    draw_rectangle(menu_pos.x, menu_pos.y, menu_size.x, menu_size.y, GRAY);

    Group::new(hash!(), menu_size)
        .position(menu_pos)
        .ui(&mut root_ui(), |ui| {
            menu_builder.label(ui, "Mode selection");

            if menu_builder.button(ui, "Singleplayer") {
                context.gamemode = crate::ui::state::GameMode::Singleplayer;
                *state = GuiState::DificultySelection;
            }

            if menu_builder.button(ui, "Multiplayer") {
                *state = ServerSelection;
            }

            if menu_builder.button(ui, "Go back") {
                *state = GuiState::MainMenu;
            }
        });

    next_frame().await;
}
