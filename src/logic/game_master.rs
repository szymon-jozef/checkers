use crate::logic::{board::board::Board, player::Player};

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
}
