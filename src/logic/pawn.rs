use uuid::Uuid;

use crate::logic::{player::Player, utils::Position};

#[derive(PartialEq, Eq, Debug)]
pub enum PawnState {
    Man(Position),
    Dame(Position),
    Captured,
}

/// Basic pawn
#[derive(Debug, PartialEq, Eq)]
pub struct Pawn {
    pub state: PawnState,
    pub owner: Uuid,
}

impl Pawn {
    pub fn new(start_pos: Position, owner: &Player) -> Self {
        Self {
            state: PawnState::Man(start_pos),
            owner: owner.id,
        }
    }
}
