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

pub async fn draw_server_selection(state: &mut GuiState, context: &mut GameContext) {
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
            menu_builder.label(ui, "Host");
            if menu_builder.button(ui, "Start") {
                context.gamemode = crate::ui::state::GameMode::Multiplayer { is_hosting: true };
            }

            menu_builder.label(ui, "Server selection");

            menu_builder.text_input(ui, &mut context.server_url_buffer);
            //context.normalise_url();

            if menu_builder.button(ui, "Connect") {
                context.gamemode = crate::ui::state::GameMode::Multiplayer { is_hosting: false };
                *state = GuiState::Connecting(connect_to_server());
                // TODO Validate url before connecting or something like that
            }

            if menu_builder.button(ui, "Go back") {
                *state = GuiState::ModeSelection;
            }
        });
}
