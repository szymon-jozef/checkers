use checkers::ui::{
    macroquad::main_menu::{draw_main_menu, get_main_menu_style},
    state::GuiState,
};
use macroquad::{
    color::{BLUE, RED, WHITE},
    math::vec2,
    miniquad::{self, conf::LinuxBackend::WaylandWithX11Fallback},
    texture::{DrawTextureParams, draw_texture_ex, load_texture},
    ui::root_ui,
    window::{Conf, clear_background, next_frame, screen_height, screen_width},
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

#[macroquad::main(window_conf)]
pub async fn main() {
    env_logger::init();

    let mut state = Default::default();
    let background = load_texture("assets/background.png").await.unwrap();

    let main_menu_style = get_main_menu_style().await;
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

        match state {
            GuiState::MainMenu => {
                draw_main_menu(&mut state).await;
            }

            GuiState::Settings => {
                clear_background(RED);
                next_frame().await;
            }
            GuiState::Game => {
                clear_background(BLUE);
                next_frame().await;
            }

            GuiState::Exit => {
                break;
            }
        }
    }
}
