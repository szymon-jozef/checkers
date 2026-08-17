use std::fmt::Display;

use uuid::Uuid;

use crate::logic::math::vector::Vector2D;

pub struct Player {
    pub name: String,
    pub id: Uuid,
    pub vertical_direction: Option<Vector2D>,
    pub end_row: Option<usize>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            id: Uuid::new_v4(),
            end_row: None,
            vertical_direction: None,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end_row_str = match self.end_row {
            Some(row) => row.to_string(),
            None => "None".to_string(),
        };

        let direction_str = match &self.vertical_direction {
            Some(direction) => direction.to_string(),
            None => "None".to_string(),
        };

        write!(
            f,
            "(name: {}, id: {}, end_row: {}, vertical_direction: {})",
            self.name, self.id, end_row_str, direction_str
        )
    }
}
