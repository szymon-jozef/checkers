use log::{error, info};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{Receiver, Sender},
};

use crate::{
    logic::game_master::GameMaster,
    network::{
        connection::{Connection, ConnectionType},
        message::{
            ClientMessage::{self, AnswerHandshake, RequestCapture, RequestMove},
            Message, ServerMessage,
        },
    },
};

// TODO! More settings!
pub struct ServerSettings {
    pub addr: SocketAddr,
    pub max_connections: usize,
    pub allow_spectators: bool,
}

pub struct Server {
    listener: TcpListener,
    connections: HashMap<SocketAddr, Sender<Message<ServerMessage>>>,
    reciever: Receiver<(SocketAddr, Message<ClientMessage>)>,
    sender: Sender<(SocketAddr, Message<ClientMessage>)>,
    game_master: Option<GameMaster>,
    settings: ServerSettings,
}

impl Server {
    pub async fn new() -> Server {
        // TODO! Should be read from a file
        let settings = ServerSettings {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6767),
            max_connections: 2,
            allow_spectators: false,
        };

        let (sender, mut reciever) =
            tokio::sync::mpsc::channel::<(SocketAddr, Message<ClientMessage>)>(1024); // random
        // number

        Server {
            listener: TcpListener::bind(settings.addr)
                .await
                .expect("Could not start the server"), // TODO! Handle this nicely, as this will
            // crash gui app
            connections: HashMap::new(),
            sender,
            reciever,
            game_master: None,
            settings,
        }
    }

    pub async fn start(&mut self) {
        self.start_listening().await;
    }

    async fn start_listening(&mut self) {
        // TODO! Maybe allow connection and then gently refuse with reason?
        while self.connections.len() < self.settings.max_connections
            && !self.settings.allow_spectators
        {
            match self.listener.accept().await {
                Ok((socket, addr)) => {
                    self.handle_new_client(socket, addr);
                }
                Err(e) => {
                    error!("Error while accepting a new connection: {}", e);
                }
            }
        }
    }

    fn handle_new_client(&mut self, socket: TcpStream, addr: SocketAddr) {
        info!("New connection from: {:?}", addr);
        let mut conn: Connection<ClientMessage, ServerMessage> =
            Connection::new(ConnectionType::Client, self.sender.clone());

        conn.delegate(socket, addr);
        let sender = conn.get_sender();

        self.connections.insert(addr, sender);

        self.listen_client(conn);
    }

    fn listen_client(&mut self, mut conn: Connection<ClientMessage, ServerMessage>) {
        tokio::spawn(async move {
            conn.start_listening().await;
        });
    }

    /// Process incoming messages
    pub async fn update(&mut self) {
        let Some(oldest_message) = self.reciever.recv().await else {
            return;
        };

        let (addr, msg) = oldest_message;

        match msg.content {
            ClientMessage::AnswerHandshake { player_name } => {
                self.process_handshake();
            }
            ClientMessage::RequestCapture { capture_path } => {
                self.process_capture();
            }
            ClientMessage::RequestMove { from, to } => {
                self.process_move();
            }
        }
    }

    /* ======= Processing messages :D helper functions ======== */

    fn process_handshake(&self) {
        todo!();
    }

    fn process_capture(&self) {
        todo!();
    }

    fn process_move(&self) {
        todo!();
    }

    async fn request_handshake(&mut self, addr: SocketAddr) {
        let msg = Message::new(ServerMessage::RequestHandshake);
        self.send_message(addr, msg).await;
    }

    async fn send_message(
        &mut self,
        addr: SocketAddr,
        msg: Result<Message<ServerMessage>, postcard::Error>,
    ) {
        let conn_opt = self.connections.get_mut(&addr);

        let Some(conn) = conn_opt else {
            error!("Could not get connection from connections list");
            return;
        };

        let _ = match msg {
            Err(e) => {
                error!("There was an error while creating message: {}", e);
                return;
            }
            Ok(msg) => conn.send(msg).await,
        };
    }
}
