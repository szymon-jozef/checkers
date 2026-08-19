use macroquad::{
    color::BLACK,
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
    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}
