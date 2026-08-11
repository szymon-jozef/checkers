use crate::logic::utils::Field;

pub enum PawnState {
    Man(Field),
    Dame(Field),
    Captured,
}

/// Basic pawn
pub struct Pawn {
    state: PawnState,
    // TODO! Change this to Player when Player is implemented
    owner: String,
}

impl Pawn {
    // TODO! Look at line 16
    /// Every new Pawn is of type Man
    pub fn new(start_pos: Field, owner: String) -> Self {
        Self {
            state: PawnState::Man(start_pos),
            owner,
        }
    }

    /// Return true if a pawn on given position is owned by the same player that this is
    // TODO! Implement
    fn is_mine(pos: Field) -> bool {
        true
    }

    pub fn can_capture(&self) -> bool {
        match &self.state {
            PawnState::Man(pos) => {
                // Man can capture only on diagonal axis going one up, left or right
                let left_pos: Field = Field {
                    row: (pos.row + 1),
                    column: (pos.column - 1),
                };
                let left_right: Field = Field {
                    row: (pos.row + 1),
                    column: (pos.column + 1),
                };
            }
            PawnState::Dame(pos) => {}
            PawnState::Captured => {
                // Captured pawns cannot capture
                return false;
            }
        }

        false
    }
}
