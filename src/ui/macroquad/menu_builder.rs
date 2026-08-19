use macroquad::{
    math::{Vec2, vec2},
    ui::{
        Ui, root_ui,
        widgets::{Button, Label},
    },
};

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

    pub fn label(&mut self, ui: &mut Ui, label: &str) {
        Label::new(label)
            .position(vec2(self.center_x, self.current_y))
            .size(self.button_size)
            .ui(ui);

        self.current_y += self.button_size.y + self.spacing;
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
