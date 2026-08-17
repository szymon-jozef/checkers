use serde::{Deserialize, Serialize};

use crate::logic::board::{board::Board, field::Field, pawn::Pawn};

/// View of the board, without any methods. Meant for sending over network
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BoardView {
    pub board: Vec<Field>,
    pub size: usize,
}

impl From<Board> for BoardView {
    fn from(value: Board) -> Self {
        Self {
            board: value.get_board(),
            size: value.size,
        }
    }
}
