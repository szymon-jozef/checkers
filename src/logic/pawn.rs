use uuid::Uuid;

use crate::logic::{player::Player, utils::Position};

#[derive(PartialEq, Eq, Debug)]
pub enum PawnState {
    Man,
    Dame,
    Captured,
}

/// Basic pawn
#[derive(Debug, PartialEq, Eq)]
pub struct Pawn {
    pub state: PawnState,
    pub owner: Uuid,
}

impl Pawn {
    pub fn new(owner: &Player) -> Self {
        Self {
            state: PawnState::Man,
            owner: owner.id,
        }
    }
}
