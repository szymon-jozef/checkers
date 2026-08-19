use macroquad::{
    color::WHITE,
    math::vec2,
    ui::{hash, root_ui},
    window::{next_frame, screen_height, screen_width},
};

use crate::ui::state::GuiState;

pub async fn draw_main_menu(state: &mut GuiState) {
    let _style = root_ui().style_builder().text_color(WHITE).font_size(64);
    let _text_style = root_ui().style_builder().text_color(WHITE).font_size(32);

    let window_size = vec2(370.0, 320.0);

    root_ui().window(
        hash!(),
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        window_size,
        |ui| {
            ui.label(vec2(80.0, -34.0), "Main Menu");
            if ui.button(vec2(65.0, 25.0), "Play") {
                *state = GuiState::Game;
            }

            if ui.button(vec2(65.0, 25.0), "Settings") {
                *state = GuiState::Settings;
            }

            if ui.button(vec2(65.0, 125.0), "Quit") {
                *state = GuiState::Exit;
            }
        },
    );

    next_frame().await;
}
