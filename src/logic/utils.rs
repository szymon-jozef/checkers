use core::fmt;

use crate::logic::pawn::Pawn;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

pub struct Field {
    pub position: Position,
    pub pawn: Option<Pawn>,
}
