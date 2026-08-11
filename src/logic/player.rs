use std::fmt::Display;

use uuid::Uuid;

pub struct Player {
    pub name: String,
    pub id: Uuid,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(name: {}, id, {})", self.name, self.id)
    }
}
