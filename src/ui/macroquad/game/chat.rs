use log::debug;
use macroquad::{
    color::{BLACK, WHITE},
    math::{Rect, vec2},
    shapes::draw_rectangle,
    text::measure_text,
    ui::{
        Skin, Ui, hash, root_ui,
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

        let font_size = (self.area.w * 0.1) as u16;

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

        let mut prev_sender: &str = "";

        Group::new(hash!(), chat_size)
            .position(chat_pos)
            .ui(&mut root_ui(), |ui| {
                for message in &self.messages {
                    if prev_sender != message.sender {
                        if prev_sender != "" {
                            // make space if it's not the first message
                            ui.label(None, "");
                        }
                        ui.label(None, &format!("[{}]", message.sender));
                    }

                    wrap_message(ui, &message.content, font_size, chat_size.x);
                    prev_sender = &message.sender;
                }
            });

        root_ui().pop_skin();
    }
}

fn wrap_message(ui: &mut Ui, msg: &String, font_size: u16, chat_width: f32) {
    let chat_width = chat_width - 20.0; // we leave some room

    let mut buffer = String::new();

    if measure_text(msg, None, font_size, 1.0).width < chat_width {
        ui.label(None, &msg);
        return;
    }

    for word in msg.split_whitespace() {
        if measure_text(word, None, font_size, 1.0).width >= chat_width {
            wrap_one_word(ui, word, font_size, chat_width);
            continue;
        }

        let test_string = if buffer.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", buffer, word)
        };

        let test_width = measure_text(&test_string, None, font_size, 1.0).width;

        if test_width < chat_width {
            buffer = test_string;
        } else {
            if !buffer.is_empty() {
                ui.label(None, &buffer);
            }

            buffer = word.to_string();
        }
    }

    if !buffer.is_empty() {
        ui.label(None, &buffer);
    }
}

fn wrap_one_word(ui: &mut Ui, msg: &str, font_size: u16, chat_width: f32) {
    if measure_text(msg, None, font_size, 1.0).width < chat_width {
        return;
    }

    let msg_half = msg.chars().count() / 2;

    let first_half: String = msg.chars().take(msg_half).collect();
    let second_half: String = msg.chars().skip(msg_half).collect();

    if measure_text(&first_half, None, font_size, 1.0).width >= chat_width {
        wrap_one_word(ui, &first_half, font_size, chat_width);
    } else {
        ui.label(None, &format!("{}-", first_half));
    }

    if measure_text(&second_half, None, font_size, 1.0).width >= chat_width {
        wrap_one_word(ui, &second_half, font_size, chat_width);
    } else {
        ui.label(None, &format!("{}-", second_half)); // TODO! Last word always has - at the end. 
    }
}
