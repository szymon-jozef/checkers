use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::logic::{math::position::Position, player::Player};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum PawnState {
    Man,
    Dame,
}

/// Basic pawn
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
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

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct CapturePath {
    pub from: Position,
    pub steps: Vec<Position>,
    pub captured_enemies: Vec<Position>,
}

impl CapturePath {
    pub fn iter(
        &self,
    ) -> std::iter::Zip<std::slice::Iter<'_, Position>, std::slice::Iter<'_, Position>> {
        self.steps.iter().zip(self.captured_enemies.iter())
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct MovePath {
    pub from: Position,
    pub available_steps: Vec<Position>,
}
