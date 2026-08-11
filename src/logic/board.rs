use crate::logic::{
    pawn::Pawn,
    player::Player,
    utils::{Field, Position},
};
use std::ops::Index;

use uuid::Uuid;

pub struct Board {
    board: Vec<Field>,
    pub size: usize,
}

impl Board {
    /// Board is always a square. Size parameter is a side of this rectangle. Defaults to 8
    pub fn new(player1: &Player, player2: &Player, size: Option<usize>) -> Self {
        let size = size.unwrap_or(8);

        let board: Vec<Field> = (0..size)
            .flat_map(|row| {
                (0..size).map(move |column| {
                    let is_row_even: bool = row % 2 == 0;
                    let is_column_even: bool = column % 2 == 0;
                    let is_valid_place_for_a_pawn: bool = is_row_even ^ is_column_even;
                    let position: Position = Position { row, column };

                    // first player's pawns
                    if row < 3 && is_valid_place_for_a_pawn {
                        Field {
                            position,
                            pawn: Some(Pawn::new(position, &player1)),
                        }
                    } else if row >= size - 3 && is_valid_place_for_a_pawn {
                        Field {
                            position,
                            pawn: Some(Pawn::new(position, &player2)),
                        }
                    } else {
                        // no pawn
                        Field {
                            position,
                            pawn: None,
                        }
                    }
                })
            })
            .collect();

        Board { board, size }
    }
}

impl Index<usize> for Board {
    type Output = [Field];

    fn index<'a>(&'a self, i: usize) -> &'a [Field] {
        let start = i * &self.size;
        let end: usize = start + &self.size;
        &self.board[start..end]
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::logic::pawn::PawnState;

    use super::*;

    #[test]
    fn new_board_test() {
        let player1: Player = Player {
            name: String::from("Morbius"),
            id: Uuid::new_v4(),
        };

        let player2: Player = Player {
            name: String::from("Milo"),
            id: Uuid::new_v4(),
        };

        let test_board: Board = Board::new(&player1, &player2, None);
        let size: usize = test_board.size;

        for row in 0..size {
            for column in 0..size {
                assert_eq!(test_board[row][column].position.row, row);
                assert_eq!(test_board[row][column].position.column, column);
            }
        }

        for row in 0..size {
            for column in 0..size {
                let is_row_even: bool = row % 2 == 0;
                let is_column_even: bool = column % 2 == 0;
                let is_valid_place_for_a_pawn: bool =
                    (is_row_even ^ is_column_even) && (row < 3 || row >= size - 3);

                if is_valid_place_for_a_pawn {
                    assert!(test_board[row][column].pawn.is_some());
                    // we unwrap, because some value has to be here, if it's not then something is wrong
                    // beside we check for that in the assert above
                    assert!(
                        test_board[row][column].pawn.as_ref().unwrap().state
                            == PawnState::Man(Position { row, column })
                    );

                    if row < 3 {
                        assert_eq!(
                            test_board[row][column].pawn.as_ref().unwrap().owner,
                            player1.id
                        )
                    } else if row >= size - 3 {
                        assert_eq!(
                            test_board[row][column].pawn.as_ref().unwrap().owner,
                            player2.id
                        )
                    }
                } else {
                    assert!(test_board[row][column].pawn.is_none());
                }
            }
        }
    }
}
