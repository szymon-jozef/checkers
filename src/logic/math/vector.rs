use core::fmt;
use std::ops::{Add, Mul, Sub};

/// 2D Vector consisting of row and column of signed types.
///
/// Has:
/// - display trait
/// - `+` | `-` | `*<usize>` overloaded
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

/// Multiplying by a scalar
impl Mul<usize> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: usize) -> Self::Output {
        Vector2D {
            row: self.row * rhs as i8,
            column: self.column * rhs as i8,
        }
    }
}
