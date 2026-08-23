use macroquad::{
    color::{BLACK, Color, GREEN, RED, WHITE},
    input::{is_mouse_button_pressed, mouse_position},
    math::{Circle, Rect, vec2},
    miniquad::window::high_dpi,
    shapes::{draw_circle, draw_circle_lines, draw_rectangle, draw_rectangle_ex},
};
use uuid::Uuid;

use crate::logic::{
    board::{
        board_view::BoardView,
        field::Field,
        pawn::{CapturePath, MovePath},
    },
    math::position::Position,
};

struct GuiField {
    rect: Rect,
    field: Field,
    pos: Position,
    is_clickable: bool,
    color: Color,
}

pub struct Board {
    rect: Rect,

    board_view: BoardView,
    fields: Vec<GuiField>,

    my_id: Uuid,

    available_captures: Option<Vec<CapturePath>>,
    available_moves: Option<Vec<MovePath>>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new(Rect::default(), Uuid::default())
    }
}

impl Board {
    pub fn new(rect: Rect, player_id: Uuid) -> Self {
        Self {
            rect,

            board_view: BoardView::default(),
            fields: vec![],

            my_id: player_id,

            available_captures: None,
            available_moves: None,
        }
    }

    pub fn update_board_view(&mut self, board_view: BoardView) {
        self.board_view = board_view;
    }

    pub fn update_available_captures(&mut self, captures: Vec<CapturePath>) {
        self.available_captures = Some(captures);
        self.available_moves = None;
    }

    pub fn update_available_moves(&mut self, moves: Vec<MovePath>) {
        self.available_moves = Some(moves);
        self.available_captures = None;
    }

    pub fn update_board_rect(&mut self, rect: Rect) -> bool {
        if self.rect != rect {
            self.rect = rect;
            return true;
        }

        false
    }

    pub fn update_state(&mut self) {
        self.fields.clear();

        let field_size = self.rect.w / self.board_view.size as f32;

        for field in &self.board_view {
            let pos: Position = field.position;

            let is_row_even: bool = pos.row % 2 == 0;
            let is_column_even: bool = pos.column % 2 == 0;
            let is_field_black: bool = is_row_even ^ is_column_even;

            let abs_x = self.rect.x + pos.column as f32 * field_size;
            let abs_y = self.rect.y + pos.row as f32 * field_size;

            let rect = Rect {
                x: abs_x,
                y: abs_y,
                w: field_size,
                h: field_size,
            };

            self.fields.push(GuiField {
                rect,
                field: field.clone(),
                is_clickable: false,
                pos,
                color: if is_field_black { BLACK } else { WHITE },
            });
        }
    }

    pub fn draw(&self) {
        for field in &self.fields {
            draw_rectangle(
                field.rect.x,
                field.rect.y,
                field.rect.w,
                field.rect.h,
                field.color,
            );

            if field.is_clickable {
                draw_rectangle(
                    field.rect.x,
                    field.rect.y,
                    field.rect.w,
                    field.rect.h,
                    GREEN,
                );
            }

            if let Some(pawn) = &field.field.pawn {
                let color = if pawn.owner == self.my_id { WHITE } else { RED }; // TODO! This should
                // be customizable

                let center = field.rect.center();

                let r = field.rect.w * 0.5;
                let thickness = 0.5;

                draw_circle(center.x, center.y, r, color);
                draw_circle_lines(center.x, center.y, r, thickness, BLACK);
            }
        }
    }

    fn highligt_current_moves(&mut self) {}

    /*
    pub fn update_current_highlight(&mut self) {
        let (mut mouse_pos_x, mut mouse_pos_y) = mouse_position();
        mouse_pos_x /= self.board_view.size as f32;
        mouse_pos_y /= self.board_view.size as f32;
    }
    */
}
