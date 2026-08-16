use core::fmt;
use std::ops::Sub;

use log::{debug, trace};
use serde::{Deserialize, Serialize};

use crate::logic::math::vector::Vector2D;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// Position on the board. Consists of row and column. Should be treated like a point in space on
/// unsigned grid.
///
/// Has:
/// - checked_add for adding with `Vector2d`
/// - is_in_range for checking if position fits on the board
/// - `Position` - `Position` = `Vector2D` operator overloaded
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
        trace!("Adding row: {} + {} = {}", self.row, delta.row, new_row);
        trace!(
            "Adding column: {} + {} = {}",
            self.column,
            delta.column,
            new_column
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

#[cfg(test)]
mod tests {
    use crate::logic::math::{position::Position, vector::Vector2D};

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_adding_positions() {
        init_logger();

        let pos1: Position = Position { row: 1, column: 1 };
        let delta_ok: Vector2D = Vector2D { row: 1, column: 1 };

        let result1: Option<Position> = pos1.checked_add(&delta_ok);
        assert!(result1.is_some());
        assert_eq!(result1.unwrap(), Position { row: 2, column: 2 });

        let delta_minus_ok: Vector2D = Vector2D {
            row: -1,
            column: -1,
        };

        let result1: Option<Position> = pos1.checked_add(&delta_minus_ok);
        assert!(result1.is_some());
        assert_eq!(result1.unwrap(), Position { row: 0, column: 0 });

        let delta_not_ok: Vector2D = Vector2D {
            row: -10,
            column: -10,
        };

        let result3: Option<Position> = pos1.checked_add(&delta_not_ok);
        assert!(result3.is_none());
    }
}
