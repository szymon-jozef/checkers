use crate::logic::pawn::Pawn;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

pub struct Field {
    pub position: Position,
    pub pawn: Option<Pawn>,
}
