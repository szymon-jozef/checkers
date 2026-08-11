use std::fmt::Display;

use uuid::Uuid;

pub struct Player {
    pub name: String,
    pub id: Uuid,
    /// Row where players pawns turn into dames
    pub end_row: usize,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            id: Uuid::new_v4(),
            end_row: 0,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(name: {}, id: {}, end_row: {})",
            self.name, self.id, self.end_row
        )
    }
}
