use core::fmt;

use crate::logic::pawn::Pawn;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    pub position: Position,
    pub pawn: Option<Pawn>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pawn) = &self.pawn {
            write!(
                f,
                "Field at pos: {}. Pawn here owned by: {}",
                self.position, pawn.owner
            )
        } else {
            write!(f, "Field at pos: {}. No pawn here.", self.position)
        }
    }
}
