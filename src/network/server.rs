use log::{error, info};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{Receiver, Sender},
};

use crate::{
    logic::game_master::GameMaster,
    network::{
        connection::{Connection, ConnectionType},
        message::{ClientMessage, Message, ServerMessage},
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
    connections: Vec<Connection<ClientMessage, ServerMessage>>,
    reciever: Receiver<Message<ClientMessage>>,
    sender: Sender<Message<ClientMessage>>,
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

        let (sender, mut reciever) = tokio::sync::mpsc::channel::<Message<ClientMessage>>(1024); // random
        // number

        Server {
            listener: TcpListener::bind(settings.addr)
                .await
                .expect("Could not start the server"), // TODO! Handle this nicely, as this will
            // crash gui app
            connections: vec![],
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
        self.connections.push(conn);
    }

    fn request_handshake(&self) {
        let conn = self.connections.last().unwrap();
        info!("Requesting handshake from: {:?}", conn);

        todo!();
    }
}
