use std::ops::Index;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl Index<usize> for BoardView {
    type Output = [Field];

    fn index<'a>(&'a self, i: usize) -> &'a [Field] {
        let start = i * &self.size;
        let end: usize = start + &self.size;
        &self.board[start..end]
    }
}

impl BoardView {
    pub fn to_string(&self, owner: Uuid) -> String {
        (0..self.size)
            .map(move |row| {
                (0..self.size)
                    .map(move |column| match &self[row][column].pawn {
                        Some(pawn) => match pawn.state {
                            super::pawn::PawnState::Man => {
                                if pawn.owner == owner {
                                    'm'
                                } else {
                                    'M'
                                }
                            }
                            super::pawn::PawnState::Dame => {
                                if pawn.owner == owner {
                                    'd'
                                } else {
                                    'D'
                                }
                            }
                        },
                        None => ' ',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
