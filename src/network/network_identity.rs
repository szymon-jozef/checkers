use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

use crate::network::message::{Message, ServerMessage};

#[derive(Deserialize, Serialize)]
pub struct NetworkIdentity {
    pub name: String,
    pub id: uuid::Uuid,
    pub is_ready: bool,
}

impl Default for NetworkIdentity {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: uuid::Uuid::nil(),
            is_ready: false,
        }
    }
}

pub struct NetworkServerIdentity {
    // this feels silly but i have no better idea rn (i've been
    // writing this since 10 a.m. and it's 9 p.m. already)
    pub identity: NetworkIdentity,
    pub sender: Sender<Message<ServerMessage>>,
}

impl NetworkServerIdentity {
    pub fn new(sender: Sender<Message<ServerMessage>>) -> Self {
        Self {
            identity: NetworkIdentity::default(),
            sender,
        }
    }
}

impl AsMut<NetworkIdentity> for NetworkServerIdentity {
    fn as_mut(&mut self) -> &mut NetworkIdentity {
        &mut self.identity
    }
}
