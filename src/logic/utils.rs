use core::fmt;

use log::debug;

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
    pub fn checked_add(&self, delta: &DeltaPosition) -> Option<Position> {
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
}

/// Vector for moving `Position`
pub struct DeltaPosition {
    pub row: i8,
    pub column: i8,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_adding_positions() {
        init_logger();

        let pos1: Position = Position { row: 1, column: 1 };
        let delta_ok: DeltaPosition = DeltaPosition { row: 1, column: 1 };

        let result1: Option<Position> = pos1.checked_add(&delta_ok);
        assert!(result1.is_some());
        assert_eq!(result1.unwrap(), Position { row: 2, column: 2 });

        let delta_minus_ok: DeltaPosition = DeltaPosition {
            row: -1,
            column: -1,
        };

        let result1: Option<Position> = pos1.checked_add(&delta_minus_ok);
        assert!(result1.is_some());
        assert_eq!(result1.unwrap(), Position { row: 0, column: 0 });

        let delta_not_ok: DeltaPosition = DeltaPosition {
            row: -10,
            column: -10,
        };

        let result3: Option<Position> = pos1.checked_add(&delta_not_ok);
        assert!(result3.is_none());
    }
}
