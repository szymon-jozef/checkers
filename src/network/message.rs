use uuid::Uuid;

use crate::logic::{
    board::{
        board::Board,
        pawn::{CapturePath, MovePath},
    },
    game_master::GameResult,
    math::position::Position,
};

pub enum ServerMessage {
    RequestHandshake,
    AcceptHandshake { player_id: Uuid },
    DeclineHandshake { reason: String },

    AvailableCaptures { captures: Vec<CapturePath> },
    AvailableMoves { moves: Vec<MovePath> },

    BroadcastBoardState { board: Board },
    BroadcastCurrentTurn { active_player: Uuid },

    GameEnd { result: GameResult },
}

pub enum ClientMessage {
    AnswerHandshake { player_name: String },

    RequestCapture { capture_path: CapturePath },
    RequestMove { from: Position, to: Position },
}
