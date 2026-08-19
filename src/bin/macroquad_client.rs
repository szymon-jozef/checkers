use checkers::ui::{macroquad::main_menu::draw_main_menu, state::GuiState};
use macroquad::{
    color::{BLUE, RED},
    miniquad::{self, conf::LinuxBackend::WaylandWithX11Fallback},
    window::{Conf, clear_background, next_frame},
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

    loop {
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
