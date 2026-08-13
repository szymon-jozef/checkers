use core::fmt;
use std::ops::Sub;

use log::debug;

use crate::logic::math::vector::Vector2D;

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

impl Position {
    pub fn checked_add(&self, delta: &Vector2D) -> Option<Position> {
        let new_row = self.row.checked_add_signed(delta.row as isize)?;
        let new_column = self.column.checked_add_signed(delta.column as isize)?;
        debug!("Adding row: {} + {} = {}", self.row, delta.row, new_row);
        debug!(
            "Adding column: {} + {} = {}",
            self.column, delta.column, new_column
        );

        Some(Position {
            row: new_row,
            column: new_column,
        })
    }

    pub fn is_in_range(&self, range: usize) -> bool {
        self.row < range && self.column < range
    }
}

impl Sub for Position {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            row: self.row as i8 - rhs.row as i8,
            column: self.column as i8 - rhs.column as i8,
        }
    }
}
