use macroquad::{
    color::{BLACK, RED, WHITE},
    math::vec2,
    shapes::{draw_circle, draw_rectangle},
};

use crate::logic::{board::board_view::BoardView, math::position::Position};

pub struct Board {
    board_view: BoardView,
}

impl From<BoardView> for Board {
    fn from(value: BoardView) -> Self {
        Self { board_view: value }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from(BoardView::default())
    }
}

impl Board {
    pub fn draw(&self) {
        let field_size = 32.0;

        for field in &self.board_view {
            let pos: Position = field.position;

            let is_row_even: bool = pos.row % 2 == 0;
            let is_column_even: bool = pos.column % 2 == 0;
            let is_field_black: bool = is_row_even ^ is_column_even;

            let abs_x = pos.column as f32 * field_size;
            let abs_y = pos.row as f32 * field_size;

            if is_field_black {
                draw_rectangle(abs_x, abs_y, field_size, field_size, BLACK);
            } else {
                draw_rectangle(abs_x, abs_y, field_size, field_size, WHITE);
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
}
