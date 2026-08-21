use macroquad::{
    color::{BLACK, GRAY, WHITE},
    math::vec2,
    shapes::draw_rectangle,
    ui::{Skin, hash, root_ui, widgets::Group},
    window::{next_frame, screen_height, screen_width},
};

use crate::ui::{macroquad::menu_builder::MenuBuilder, state::GuiState};

pub async fn draw_main_menu(state: &mut GuiState) {
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
            menu_builder.label(ui, "Main menu");

            if menu_builder.button(ui, "Play") {
                *state = GuiState::ModeSelection;
            }

            if menu_builder.button(ui, "Settings") {
                *state = GuiState::Settings;
            }

            if menu_builder.button(ui, "Quit") {
                *state = GuiState::Exit;
            }
        });
}
