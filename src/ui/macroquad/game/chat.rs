use macroquad::{
    color::WHITE,
    math::{Rect, vec2},
    shapes::draw_rectangle,
    ui::{hash, root_ui, widgets::InputText},
    window::{screen_height, screen_width},
};
use tokio::sync::mpsc::Sender;

use crate::ui::macroquad::game::game::GuiCommands;

struct Message {
    sender: String,
    content: String,
}

#[derive(Default)]
pub struct Chat {
    area: Rect,
    messages: Vec<Message>,
    message_buffer: String,
}

impl Chat {
    pub fn update(&mut self, game_area: &Rect) {
        self.area = Rect {
            x: game_area.w,
            y: 0.0,
            w: screen_width() * 0.2,
            h: screen_height(),
        };
    }

    pub fn push_message(&mut self, sender: String, content: String) {
        self.messages.push(Message { sender, content });
    }

    pub fn send_message(&mut self, cmd_sender: Sender<GuiCommands>) {
        if !self.message_buffer.is_empty() {
            let _ =  // TODO! Check the message before sending
                cmd_sender
                .try_send(GuiCommands::Send(self.message_buffer.clone()));
            self.message_buffer.clear();
        }
    }

    pub fn draw(&mut self) {
        draw_rectangle(self.area.x, self.area.y, self.area.w, self.area.h, WHITE);

        let input_size = vec2(self.area.w, self.area.h * 0.1);
        let input_pos = vec2(self.area.x, self.area.h - input_size.y);

        InputText::new(hash!())
            .position(input_pos)
            .size(input_size)
            .ui(&mut root_ui(), &mut self.message_buffer);
    }
}
