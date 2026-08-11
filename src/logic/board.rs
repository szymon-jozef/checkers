use crate::logic::utils::Field;
use std::ops::Index;

pub struct Board {
    board: Vec<Field>,
    pub size: usize,
}

impl Board {
    /// Board is always a square. Size parameter is a side of this rectangle. Defaults to 8
    pub fn new(size: Option<usize>) -> Self {
        let size = size.unwrap_or(8);

        let board: Vec<Field> = (0..size)
            .flat_map(|row| (0..size).map(move |column| Field { row, column }))
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
    use super::*;

    #[test]
    fn new_board_test() {
        let test_board: Board = Board::new(None);
        let size: usize = test_board.size;

        for row in 0..size {
            for column in 0..size {
                assert_eq!(test_board[row][column].row, row);
                assert_eq!(test_board[row][column].column, column);
            }
        }
    }
}
