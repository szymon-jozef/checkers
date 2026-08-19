use macroquad::{
    color::WHITE,
    math::{Rect, Vec2, vec2},
    prelude::ImageFormat,
    texture::Image,
    ui::{Layout, Skin, Ui, hash, root_ui, widgets::Button},
    window::{next_frame, screen_height, screen_width},
};

use crate::ui::state::GuiState;

pub struct MenuBuilder {
    center_x: f32,
    current_y: f32,
    button_size: Vec2,
    spacing: f32,
}

impl MenuBuilder {
    pub fn new(window_width: f32, start_y: f32) -> Self {
        let button_size = vec2(200.0, 50.0);
        Self {
            center_x: (window_width - button_size.x) / 2.0,
            current_y: start_y,
            button_size,
            spacing: 15.0,
        }
    }

    pub fn button(&mut self, ui: &mut Ui, label: &str) -> bool {
        let clicked = Button::new(label)
            .position(vec2(self.center_x, self.current_y))
            .size(self.button_size)
            .ui(ui);

        self.current_y += self.button_size.y + self.spacing;

        clicked
    }
}

pub async fn draw_main_menu(state: &mut GuiState) {
    let window_size = vec2(1200.0, 620.0);
    let mut menu_builder = MenuBuilder::new(screen_width(), 10.0);

    root_ui().window(
        hash!(),
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        window_size,
        |ui| {
            ui.label(None, "Main Menu");

            if menu_builder.button(ui, "Play") {
                *state = GuiState::Game;
            }

            if menu_builder.button(ui, "Settings") {
                *state = GuiState::Settings;
            }

            if menu_builder.button(ui, "Quit") {
                *state = GuiState::Exit;
            }
        },
    );

    next_frame().await;
}
