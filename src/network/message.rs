use std::net::SocketAddr;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::logic::{
    board::{
        board::Board,
        pawn::{CapturePath, MovePath},
    },
    game_master::GameResult,
    math::position::Position,
};

use postcard::{from_bytes, to_allocvec};

pub trait MessageLike {
    fn to_bits(&self) -> Result<Vec<u8>, postcard::Error>;
    fn from_bits(bits: &[u8]) -> Result<Self, postcard::Error>
    where
        Self: Sized;
}

impl<T> MessageLike for T
where
    T: DeserializeOwned + Serialize,
{
    fn to_bits(&self) -> Result<Vec<u8>, postcard::Error> {
        to_allocvec(self)
    }

    fn from_bits(bits: &[u8]) -> Result<Self, postcard::Error> {
        from_bytes(bits)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
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

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub enum ClientMessage {
    AnswerHandshake { player_name: String },
    SignalReadiness,

    ConnectionDead { addr: SocketAddr },

    RequestCapture { capture_path: CapturePath },
    RequestMove { from: Position, to: Position },

    TextMessage(String),
}

pub struct Message<T>
where
    T: MessageLike,
{
    pub size: u32,
    pub content: T,
}

impl<T> Message<T>
where
    T: MessageLike,
{
    pub fn new(content: T) -> Result<Message<T>, postcard::Error> {
        let size: usize = content.to_bits()?.len();

        Ok(Message {
            size: size as u32,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_message_serde() {
        let server_msg_content: ServerMessage = ServerMessage::RequestHandshake;
        let msg: Message<ServerMessage> =
            Message::new(server_msg_content.clone()).expect("Couldn't serialize message");

        assert_eq!(1, msg.size);

        let bits = msg
            .content
            .to_bits()
            .expect("Could not convert message content to bits!");

        let recovered_message: ServerMessage =
            MessageLike::from_bits(&bits).expect("Could not recover the message");

        assert_eq!(recovered_message, server_msg_content);
    }

    #[test]
    fn test_client_message_serde() {
        let from: Position = Position { row: 0, column: 0 };
        let to: Position = Position { row: 1, column: 1 };

        let client_msg_content: ClientMessage = ClientMessage::RequestMove { from, to };

        let msg = Message::new(client_msg_content.clone()).expect("Could not serialize message");

        assert_eq!(5, msg.size);

        let bits = msg
            .content
            .to_bits()
            .expect("Could not convert the message into bits!");

        let recovered_message: ClientMessage =
            MessageLike::from_bits(&bits).expect("Could not recover the message!");

        assert_eq!(recovered_message, client_msg_content);
    }
}
