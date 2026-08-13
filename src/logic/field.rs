use std::fmt;

use crate::logic::{math::position::Position, pawn::Pawn};

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
