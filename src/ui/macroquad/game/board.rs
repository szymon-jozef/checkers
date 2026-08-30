use std::{
    ops::{Index, IndexMut},
    slice::IterMut,
};

use log::{debug, info};
use macroquad::{
    color::{BLACK, Color, GREEN, ORANGE, RED, WHITE},
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

const MOVE_HIGHLITGHT_COLOR: Color = GREEN;
const CAPTURE_HIGHLITGHT_COLOR: Color = ORANGE;

#[derive(Default)]
struct GuiField {
    rect: Rect,
    field: Field,
    pos: Position,
    is_clickable: bool,
    color: Color,
    is_blac: bool,
}

#[derive(Default)]
struct GuiFieldVec {
    gui_fields: Vec<GuiField>,
}

impl GuiFieldVec {
    pub fn push(&mut self, input: GuiField) {
        self.gui_fields.push(input);
    }

    pub fn clear(&mut self) {
        self.gui_fields.clear();
    }

    pub fn size(&self) -> usize {
        self.gui_fields.len()
    }
}

impl<'a> IntoIterator for &'a GuiFieldVec {
    type Item = &'a GuiField;
    type IntoIter = std::slice::Iter<'a, GuiField>;

    fn into_iter(self) -> Self::IntoIter {
        self.gui_fields.iter()
    }
}

impl<'a> IntoIterator for &'a mut GuiFieldVec {
    type Item = &'a mut GuiField;
    type IntoIter = IterMut<'a, GuiField>;

    fn into_iter(self) -> Self::IntoIter {
        self.gui_fields.iter_mut()
    }
}

impl Index<Position> for GuiFieldVec {
    type Output = GuiField;

    fn index(&self, index: Position) -> &Self::Output {
        self.gui_fields
            .iter()
            .find(|field| field.pos == index)
            .unwrap()
    }
}

impl IndexMut<Position> for GuiFieldVec {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.gui_fields
            .iter_mut()
            .find(|field| field.pos == index)
            .unwrap()
    }
}

#[derive(Default)]
enum BoardState {
    #[default]
    View,
    ChoosePawn,
    ChooseMove(Position),
    ChooseCapture(Position),
}

pub struct Board {
    rect: Rect,

    board_view: BoardView,
    fields: GuiFieldVec,

    state: BoardState,

    my_id: Uuid,
    is_my_turn: bool,

    available_captures: Option<Vec<CapturePath>>,
    available_moves: Option<Vec<MovePath>>,

    field_size: f32,
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
            fields: GuiFieldVec::default(),

            state: BoardState::default(),

            my_id: player_id,
            is_my_turn: false,

            available_captures: None,
            available_moves: None,

            field_size: 32.0,
        }
    }

    pub fn update(&mut self) {
        self.update_dimensions();
        self.reset_color();

        let clicked_field = self.get_clicked_field();

        match self.state {
            BoardState::View => {
                if self.is_my_turn {
                    self.state = BoardState::ChoosePawn;
                }
            }

            BoardState::ChoosePawn => {
                if let Some(clicked_field) = clicked_field {
                    self.delegate_move(clicked_field.pos);
                }

                if self.is_back_move_wanted_by_player() {
                    self.state = BoardState::View;
                }

                self.highlight_current_moves();
            }

            BoardState::ChooseMove(from) => {
                if self.is_back_move_wanted_by_player() {
                    self.state = BoardState::ChoosePawn;
                }

                self.highlight_move_paths(from);
            }

            BoardState::ChooseCapture(from) => {
                if self.is_back_move_wanted_by_player() {
                    self.state = BoardState::ChoosePawn;
                }
            }
        }
    }

    fn delegate_move(&mut self, from: Position) {
        if let Some(capture_path) = &self.available_captures
            && capture_path.iter().any(|path| path.from == from)
        {
            self.state = BoardState::ChooseCapture(from);
            return;
        }

        if let Some(available_moves) = &self.available_moves
            && available_moves.iter().any(|path| path.from == from)
        {
            self.state = BoardState::ChooseMove(from);
            return;
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
        self.state = BoardState::ChoosePawn;

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

    /// Recreate fields vec with new dimensions. Should be used when new board_view arrives
    pub fn repopulate(&mut self) {
        self.fields.clear();

        self.field_size = self.rect.w / self.board_view.size as f32;

        for field in &self.board_view {
            let pos: Position = field.position;

            let is_row_even: bool = pos.row % 2 == 0;
            let is_column_even: bool = pos.column % 2 == 0;
            let is_field_black: bool = is_row_even ^ is_column_even;

            let abs_x = self.rect.x + pos.column as f32 * self.field_size;
            let abs_y = self.rect.y + pos.row as f32 * self.field_size;

            let rect = Rect {
                x: abs_x,
                y: abs_y,
                w: self.field_size,
                h: self.field_size,
            };

            self.fields.push(GuiField {
                rect,
                field: field.clone(),
                is_clickable: false,
                pos,
                color: if is_field_black { BLACK } else { WHITE },
                is_blac: is_field_black,
            });
        }
    }

    /// Updates dimensions of graphical squares
    pub fn update_dimensions(&mut self) {
        for field in &mut self.fields {
            let pos: Position = field.pos;

            let abs_x = self.rect.x + pos.column as f32 * self.field_size;
            let abs_y = self.rect.y + pos.row as f32 * self.field_size;

            let new_rect = Rect {
                x: abs_x,
                y: abs_y,
                w: self.field_size,
                h: self.field_size,
            };

            field.rect = new_rect;
        }
    }

    fn reset_color(&mut self) {
        for field in &mut self.fields {
            field.color = if field.is_blac { BLACK } else { WHITE };
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

    fn get_clicked_field(&self) -> Option<&GuiField> {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_point = vec2(mouse_x, mouse_y);

        if is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
            for field in &self.fields {
                if field.rect.contains(mouse_point) {
                    return Some(field);
                }
            }
        }
        None
    }

    fn is_back_move_wanted_by_player(&self) -> bool {
        is_mouse_button_pressed(macroquad::input::MouseButton::Right)
    }

    pub fn set_my_turn(&mut self, is_my_turn: bool) {
        self.is_my_turn = is_my_turn;
    }

    fn highlight_current_moves(&mut self) {
        let Some(available_moves) = &self.available_moves else {
            return;
        };

        let mut from: Vec<Position> = vec![];

        for available_move in available_moves {
            from.push(available_move.from);
        }

        for pos in from {
            self.fields[pos].color = MOVE_HIGHLITGHT_COLOR;
        }
    }

    fn highlight_move_paths(&mut self, from: Position) {
        let Some(available_moves) = &self.available_moves else {
            return;
        };

        for available_move in available_moves {
            if available_move.from == from {
                for step in &available_move.available_steps {
                    self.fields[*step].color = MOVE_HIGHLITGHT_COLOR;
                }
            }
        }
    }
}
