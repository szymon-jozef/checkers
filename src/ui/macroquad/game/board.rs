use macroquad::{
    color::{BLACK, GREEN, RED, WHITE},
    input::{is_mouse_button_pressed, mouse_position},
    math::{Rect, vec2},
    miniquad::window::high_dpi,
    shapes::{draw_circle, draw_rectangle, draw_rectangle_ex},
};
use uuid::Uuid;

use crate::logic::{
    board::{
        board_view::BoardView,
        pawn::{CapturePath, MovePath},
    },
    math::position::Position,
};

pub struct Board {
    board_view: BoardView,

    my_id: Uuid,
    current_highlight: Option<Position>,

    available_captures: Option<Vec<CapturePath>>,
    available_moves: Option<Vec<MovePath>>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new(Uuid::default())
    }
}

impl Board {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            board_view: BoardView::default(),
            my_id: player_id,
            current_highlight: None,

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

    pub fn draw(&mut self) {
        let field_size = 32.0;

        for field in &self.board_view {
            let pos: Position = field.position;

            let is_row_even: bool = pos.row % 2 == 0;
            let is_column_even: bool = pos.column % 2 == 0;
            let is_field_black: bool = is_row_even ^ is_column_even;

            let abs_x = pos.column as f32 * field_size;
            let abs_y = pos.row as f32 * field_size;

            let (mouse_pos_x, mouse_pos_y) = mouse_position();

            if abs_x < mouse_pos_x
                && mouse_pos_x <= abs_x + field_size
                && abs_y < mouse_pos_y
                && mouse_pos_y <= abs_y + field_size
                && is_mouse_button_pressed(macroquad::input::MouseButton::Left)
            {
                self.current_highlight = Some(pos);
            }

            if is_field_black {
                draw_rectangle(abs_x, abs_y, field_size, field_size, BLACK);
            } else {
                draw_rectangle(abs_x, abs_y, field_size, field_size, WHITE);
            }

            if let Some(hihglith) = self.current_highlight {
                let x = hihglith.column as f32 * field_size;
                let y = hihglith.row as f32 * field_size;

                draw_rectangle(x, y, field_size, field_size, GREEN);
            }

            if field.pawn.is_some() {
                // draw something for now
                draw_circle(
                    abs_x + field_size * 0.5,
                    abs_y + field_size * 0.5,
                    16.0,
                    RED,
                );
            }
        }
    }

    /*
    pub fn update_current_highlight(&mut self) {
        let (mut mouse_pos_x, mut mouse_pos_y) = mouse_position();
        mouse_pos_x /= self.board_view.size as f32;
        mouse_pos_y /= self.board_view.size as f32;
    }
    */
}
