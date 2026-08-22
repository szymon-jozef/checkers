use log::debug;
use macroquad::{
    color::{BLACK, WHITE},
    math::{Rect, vec2},
    shapes::draw_rectangle,
    ui::{
        Skin, hash, root_ui,
        widgets::{Group, InputText, Label},
    },
    window::{screen_height, screen_width},
};
use tokio::sync::mpsc::Sender;

use crate::ui::macroquad::game::game::GuiCommands;

#[derive(Debug)]
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
        draw_rectangle(self.area.x, self.area.y, self.area.w, self.area.h, BLACK);

        let input_size = vec2(self.area.w, self.area.h * 0.1);
        let input_pos = vec2(self.area.x, self.area.h - input_size.y);

        InputText::new(hash!())
            .position(input_pos)
            .size(input_size)
            .ui(&mut root_ui(), &mut self.message_buffer);

        let chat_size = vec2(self.area.w, self.area.h - input_size.y);
        let chat_pos = vec2(self.area.x, self.area.y);

        let font_size = 12;

        let label_style = root_ui()
            .style_builder()
            .font_size(font_size)
            .text_color(WHITE)
            .build();

        let chat_skin: Skin = Skin {
            label_style,
            ..root_ui().default_skin()
        };

        root_ui().push_skin(&chat_skin);

        Group::new(hash!(), chat_size)
            .position(chat_pos)
            .ui(&mut root_ui(), |ui| {
                for message in &self.messages {
                    ui.label(None, &format!("[{}] {}", message.sender, message.content));
                }
            });

        root_ui().pop_skin();
    }
}
