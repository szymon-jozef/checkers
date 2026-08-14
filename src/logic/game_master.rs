use log::warn;

use crate::logic::{
    board::{
        board::Board,
        pawn::{CapturePath, MovePath},
    },
    math::position::Position,
    player::Player,
};

pub enum GameResult {
    Lost(uuid::Uuid),
    Draw,
}

struct PlayerList {
    pub players: [Player; 2],
    current_turn: usize,
    size: usize,
}

impl PlayerList {
    pub fn new(player1_name: String, player2_name: String) -> Self {
        PlayerList {
            players: [Player::new(player1_name), Player::new(player2_name)],
            current_turn: 0,
            size: 2,
        }
    }

    pub fn change_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.size;
    }

    pub fn get_current_turn(&self) -> &Player {
        &self.players[self.current_turn]
    }

    pub fn get_passive_player(&self) -> &Player {
        &self.players[(self.current_turn + 1) % self.size]
    }
}

pub struct GameMaster {
    players: PlayerList,
    board: Board,
}

impl GameMaster {
    pub fn new(player1_name: String, player2_name: String) -> Self {
        let mut players: PlayerList = PlayerList::new(player1_name, player2_name);
        let [player1, player2] = &mut players.players;

        let board = Board::new(player1, player2, None);

        GameMaster { players, board }
    }

    pub fn get_current_player_captures(&self) -> Vec<CapturePath> {
        self.board
            .get_player_pawns_positions(self.players.get_current_turn())
            .into_iter()
            .filter_map(|pos| {
                self.board
                    .get_available_captures(pos, self.players.get_current_turn())
            })
            .flatten()
            .collect()
    }

    pub fn get_current_player_moves(&self) -> Vec<MovePath> {
        self.board
            .get_player_pawns_positions(self.players.get_current_turn())
            .into_iter()
            .filter_map(|pos| {
                self.board
                    .get_available_moves(pos, self.players.get_current_turn())
            })
            .flatten()
            .collect()
    }

    fn is_player_lost(&self, player: &Player) -> bool {
        let player_pawns = self.board.get_player_pawns_positions(player);

        player_pawns.len() == 0
            || player_pawns.iter().all(|pos| {
                self.board.get_available_moves(*pos, player).is_none()
                    && self.board.get_available_captures(*pos, player).is_none()
            })
    }

    pub fn get_game_result(&self) -> Option<GameResult> {
        let active: &Player = self.players.get_current_turn();
        let passive: &Player = self.players.get_passive_player();

        let active_lost: bool = self.is_player_lost(active);
        let passive_lost: bool = self.is_player_lost(passive);

        if active_lost && passive_lost {
            Some(GameResult::Draw)
        } else if active_lost {
            Some(GameResult::Lost(active.id))
        } else if passive_lost {
            Some(GameResult::Lost(passive.id))
        } else {
            None
        }
    }

    pub fn move_pawn(&mut self, from: Position, to: Position) -> bool {
        if self.get_current_player_moves().is_empty() {
            warn!(
                "Player: {} tried moving, but they don't have any moves available!",
                self.players.get_current_turn().id
            );
            return false;
        }

        if !self.get_current_player_captures().is_empty() {
            warn!(
                "Player: {} tried moving, but he has captures available!",
                self.players.get_current_turn().id
            );
            return false;
        }

        if self
            .board
            .move_pawn(self.players.get_current_turn(), from, to)
        {
            self.players.change_turn();
            return true;
        }
        false
    }

    pub fn capture(&mut self, path: &CapturePath) -> bool {
        if self.get_current_player_captures().is_empty() {
            warn!(
                "Player: {} tried capturing, but he doesn't have any captures available!",
                self.players.get_current_turn().id
            );
            return false;
        }

        if self.board.capture(path, self.players.get_current_turn()) {
            self.players.change_turn();
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use rand::seq::IndexedMutRandom;
    use rand::seq::IndexedRandom;
    use rand::seq::SliceRandom;

    #[test]
    fn test_game_flow() {
        let mut master: GameMaster = GameMaster::new("Morbius".to_string(), "Milo".to_string());
        let mut turns = 0;
        let max_turns = 2000;

        while turns < max_turns {
            if let Some(result) = master.get_game_result() {
                match result {
                    GameResult::Lost(loser_id) => {
                        info!("Player: {} lost! Ending the loop", loser_id);
                    }
                    GameResult::Draw => {
                        info!("Game ended in a draw! Ending the loop");
                    }
                }
                break;
            }

            let mut rng = rand::rng();

            let available_captures = master.get_current_player_captures();

            if !available_captures.is_empty() {
                let capture = available_captures.choose(&mut rng).unwrap();
                master.capture(capture);
                turns += 1;
                continue;
            }

            let mut available_moves = master.get_current_player_moves();
            if !available_moves.is_empty() {
                let path = available_moves.choose_mut(&mut rng).unwrap();
                let from = path.from;
                let to = path.available_steps.choose_mut(&mut rng).unwrap();

                master.move_pawn(from, *to);
                turns += 1;
            }
        }

        assert!(turns < max_turns);
    }
}
