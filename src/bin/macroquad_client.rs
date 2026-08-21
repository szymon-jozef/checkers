use checkers::ui::{
    macroquad::{
        difficulty_selection::draw_dificulty_selection, main_menu::draw_main_menu,
        mode_selection::draw_mode_selection, server_selection::draw_server_selection,
    },
    state::{GameContext, GuiState},
};
use macroquad::{
    color::{BLACK, WHITE},
    math::vec2,
    miniquad::{self, conf::LinuxBackend::WaylandWithX11Fallback},
    text::draw_text,
    texture::{DrawTextureParams, draw_texture_ex, load_texture},
    ui::{Skin, root_ui},
    window::{Conf, next_frame, screen_height, screen_width},
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Checkers".to_owned(),
        window_resizable: true,
        platform: miniquad::conf::Platform {
            linux_backend: WaylandWithX11Fallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub async fn get_general_style() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .text_color(WHITE)
        .font_size(64)
        .build();

    let button_style = root_ui()
        .style_builder()
        .text_color(BLACK)
        .font_size(32)
        .build();

    Skin {
        label_style,
        button_style,
        ..root_ui().default_skin()
    }
}

#[macroquad::main(window_conf)]
pub async fn main() {
    env_logger::init();

    let mut state = Default::default();
    let mut context = GameContext::default();

    let background = load_texture("assets/background.png").await.unwrap();

    let main_menu_style = get_general_style().await;
    root_ui().push_skin(&main_menu_style);

    loop {
        draw_texture_ex(
            &background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..DrawTextureParams::default()
            },
        );

        match &state {
            GuiState::MainMenu => {
                draw_main_menu(&mut state).await;
            }

            GuiState::ModeSelection => {
                draw_mode_selection(&mut state, &mut context).await;
            }

            GuiState::DificultySelection => {
                draw_dificulty_selection(&mut state, &mut context).await;
            }

            GuiState::ServerSelection => {
                draw_server_selection(&mut state, &mut context).await;
            }

            GuiState::Connecting(receiver) => {
                draw_text("Connecting...", 0.0, 0.0, 64.0, BLACK);

                match receiver.try_recv() {
                    Ok(client) => {
                        if let Some(client) = client {
                            draw_text("Connected!", 0.0, 0.0, 64.0, BLACK);
                            state = GuiState::Game(client);
                        } else {
                            draw_text("Could not connect!", 0.0, 0.0, 64.0, BLACK);
                        }
                    }
                    Err(_) => {
                        draw_text("Error", 0.0, 0.0, 64.0, BLACK);
                    }
                }
            }

            GuiState::Settings => {
                todo!();
            }

            GuiState::Game(client) => {
                draw_text("Gamer time", 100.0, 100.0, 64.0, BLACK);
            }

            GuiState::Exit => {
                break;
            }
        }
        next_frame().await;
    }
}
