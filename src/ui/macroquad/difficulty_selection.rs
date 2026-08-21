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
        GuiState::{self},
        connect_to_server,
    },
};

pub async fn draw_dificulty_selection(state: &mut GuiState, context: &mut GameContext) {
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
            menu_builder.label(ui, "Difficulty Selection");

            // TODO! refactor this ugly ass code

            if menu_builder.button(ui, "Easy") {
                context.difficulty = crate::super_advanced_ai::BotDificulty::Easy;
                *state = GuiState::Connecting(connect_to_server(context.clone()));
            }

            if menu_builder.button(ui, "Medium") {
                context.difficulty = crate::super_advanced_ai::BotDificulty::Normal;
                *state = GuiState::Connecting(connect_to_server(context.clone()));
            }

            if menu_builder.button(ui, "Hard") {
                context.difficulty = crate::super_advanced_ai::BotDificulty::Hard;
                *state = GuiState::Connecting(connect_to_server(context.clone()));
            }

            if menu_builder.button(ui, "Go back") {
                *state = GuiState::ModeSelection;
            }
        });
}
