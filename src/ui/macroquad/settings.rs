use log::error;
use macroquad::{
    color::GRAY,
    math::vec2,
    shapes::draw_rectangle,
    ui::{hash, root_ui, widgets::Group},
    window::{screen_height, screen_width},
};

use crate::{
    settings::{
        client_settings::ClientSettings, general_settings::SettingsLike,
        server_settings::ServerSettings,
    },
    ui::{
        macroquad::menu_builder::MenuBuilder,
        state::{GameContext, GuiState},
    },
};

pub async fn draw_settings(
    state: &mut GuiState,
    context: &mut GameContext,
    client_settings: &mut ClientSettings,
    server_settings: &mut ServerSettings,
) {
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
            // Client
            menu_builder.label(ui, "Game settings");

            menu_builder.label(ui, "Name");
            menu_builder.text_input(ui, "name input", &mut client_settings.name);

            menu_builder.label(ui, "Default server url");
            menu_builder.text_input(ui, "url input", &mut context.server_url_buffer);

            menu_builder.label(ui, "");

            // Server
            menu_builder.label(ui, "Server settings");
            menu_builder.label(ui, "TODO");

            if let Some(error_msg) = &context.latest_error_message {
                menu_builder.label(ui, &error_msg);
            }

            // Exit
            if menu_builder.button(ui, "Save and back") {
                client_settings.server_url = match context.server_url_buffer.parse() {
                    Ok(result) => result,

                    Err(e) => {
                        context.latest_error_message =
                            Some(format!("Error while parsing url: {}", e));
                        return;
                    }
                };

                if let Err(e) = client_settings.save_to_file() {
                    context.latest_error_message = Some(format!("Error: {}", e));
                    error!("Error while saving client settings to file: {}", e);
                    return;
                }

                if let Err(e) = server_settings.save_to_file() {
                    context.latest_error_message = Some(format!("Error: {}", e));
                    error!("Error while saving server settings to file: {}", e);
                    return;
                }

                context.latest_error_message = None;
                *state = GuiState::MainMenu;
            }
        });
}
