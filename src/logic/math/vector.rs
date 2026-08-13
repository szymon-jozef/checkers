use core::fmt;
use std::ops::{Add, Mul, Sub};

/// Vector for moving `Position`
#[derive(Debug, Copy, Clone)]
pub struct Vector2D {
    pub row: i8,
    pub column: i8,
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            row: self.row + rhs.row,
            column: self.column + rhs.column,
        }
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            row: self.row - rhs.row,
            column: self.column - rhs.column,
        }
    }
}

impl Mul<usize> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: usize) -> Self::Output {
        Vector2D {
            row: self.row * rhs as i8,
            column: self.column * rhs as i8,
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
