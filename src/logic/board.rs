use crate::logic::{
    pawn::{self, Pawn, PawnState},
    player::Player,
    utils::{Field, Position},
};
use std::ops::{Index, IndexMut};

use log::{debug, info, warn};

pub struct Board {
    board: Vec<Field>,
    pawns_positions: Vec<Position>,
    pub size: usize,
}

impl Board {
    /// Board is always a square. Size parameter is a side of this square. Defaults to 8
    pub fn new(player1: &mut Player, player2: &mut Player, size: Option<usize>) -> Self {
        let size = size.unwrap_or(8);

        info!("Creating new board of size: {}", size);
        player1.end_row = size - 1;
        player2.end_row = 0;

        let p1_ref: &Player = &*player1;
        let p2_ref: &Player = &*player2;

        let board: Vec<Field> = (0..size)
            .flat_map(|row| {
                (0..size).map(move |column| {
                    let is_row_even: bool = row % 2 == 0;
                    let is_column_even: bool = column % 2 == 0;
                    let is_valid_place_for_a_pawn: bool = is_row_even ^ is_column_even;
                    let position: Position = Position { row, column };

                    let current_player: Option<&Player> = if row < 3 {
                        Some(p1_ref)
                    } else if row >= size - 3 {
                        Some(p2_ref)
                    } else {
                        None
                    };

                    if let Some(player) = current_player
                        && is_valid_place_for_a_pawn
                    {
                        debug!("Placing new pawn at: {}, owned by: {}", position, player);
                        Field {
                            position,
                            pawn: Some(Pawn::new(player)),
                        }
                    } else {
                        debug!("Not placing any pawn at: {}", position);
                        Field {
                            position,
                            pawn: None,
                        }
                    }
                })
            })
            .collect();

        let pawns_positions: Vec<Position> = board
            .iter()
            .filter_map(|field| field.pawn.as_ref().map(|_| field.position))
            .collect();

        Board {
            board,
            size,
            pawns_positions,
        }
    }

    /// Checks if move from position to position is valid.
    /// It returns true if you want to move pawn from position to position where there is no pawn
    ///
    /// Doesn't check turn
    ///
    /// Not meant for capturing
    fn is_move_valid(&self, from: &Position, to: &Position) -> bool {
        if let Some(_) = &self[from.row][from.column].pawn {
            self[to.row][to.column].pawn.is_none()
        } else {
            false
        }
    }

    fn is_player_owner_of_the_pawn(&self, player: &Player, pawn: &Pawn) -> bool {
        player.id == pawn.owner
    }

    /// Returns true if move was successful
    pub fn move_pawn(&mut self, player: &Player, from: Position, to: Position) -> bool {
        if !self.is_move_valid(&from, &to) {
            warn!("Tried invalid move: {} -> {}", from, to);
            return false;
        }

        if !self.is_player_owner_of_the_pawn(&player, self[from].pawn.as_ref().unwrap()) {
            warn!("Player {} tried moving a pawn which he doesn own!", player);
            debug!("Tried moving from {} to {}", from, to);
            return false;
        }

        // we can safely unwrap, because if there's not pawn function returns false earlier
        let mut mowing_pawn: Pawn = self[from].pawn.take().unwrap();

        if to.row == player.end_row {
            mowing_pawn.state = super::pawn::PawnState::Dame;
        }

        self[to.row][to.column].pawn = Some(mowing_pawn);

        true
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

impl Index<Position> for Board {
    type Output = Field;

    fn index(&self, index: Position) -> &Self::Output {
        &self[index.row][index.column]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut [Field] {
        let start = i * &self.size;
        let end: usize = start + &self.size;
        &mut self.board[start..end]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self[index.row][index.column]
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::pawn::PawnState;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn new_board_test() {
        init_logger();

        let mut player1: Player = Player::new("Morbius".to_string());
        let mut player2: Player = Player::new("Milo".to_string());

        let test_board: Board = Board::new(&mut player1, &mut player2, None);
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
                    assert!(test_board[row][column].pawn.as_ref().unwrap().state == PawnState::Man);

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

    #[test]
    fn test_overloaded_operators() {
        init_logger();

        let mut player1: Player = Player::new("Morbius".to_string());
        let mut player2: Player = Player::new("Milo".to_string());

        let test_board: Board = Board::new(&mut player1, &mut player2, None);

        let row1 = 0;
        let column1 = 0;

        let position1: Position = Position {
            row: row1,
            column: column1,
        };

        let row2 = row1;
        let column2 = 1;

        let position2: Position = Position {
            row: row2,
            column: column2,
        };

        assert_eq!(test_board[row1][column1], test_board[position1]);
        assert_eq!(test_board[row2][column2], test_board[position2]);

        assert!(test_board[position1].pawn.is_none());
        assert!(test_board[position2].pawn.is_some());
    }

    #[test]
    fn test_moving_pawn() {
        init_logger();

        let mut player1: Player = Player::new("Morbius".to_string());
        let mut player2: Player = Player::new("Milo".to_string());

        let mut test_board: Board = Board::new(&mut player1, &mut player2, None);

        // we cant move pawn into another pawn; this should fail
        assert!(!test_board.move_pawn(
            &player1,
            Position { row: 0, column: 1 },
            Position { row: 1, column: 0 },
        ));

        // player can't move pawn that he doesn't own; this should fail
        assert!(!test_board.move_pawn(
            &player2,
            Position { row: 2, column: 1 },
            Position { row: 3, column: 0 },
        ));

        // walid move
        assert!(test_board.move_pawn(
            &player1,
            Position { row: 2, column: 1 },
            Position { row: 3, column: 0 }
        ));

        assert!(test_board[3][0].pawn.is_some());
        assert_eq!(test_board[3][0].pawn.as_ref().unwrap().owner, player1.id);
        assert!(test_board[2][1].pawn.is_none());
    }
}
