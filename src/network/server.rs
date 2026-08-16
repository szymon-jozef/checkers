use log::{error, info, warn};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::{
    net::TcpListener,
    sync::mpsc::{Receiver, Sender},
};

use crate::{
    logic::game_master::GameMaster,
    network::{
        connection::{Connection, ConnectionType},
        message::{
            ClientMessage::{self},
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

#[derive(PartialEq)]
enum ServerStage {
    Lobby,
    Game,
    End,
}

struct ServerState {
    ready_count: usize,
    stage: ServerStage,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            ready_count: 0,
            stage: ServerStage::Lobby,
        }
    }
}

pub struct Server {
    listener: Option<TcpListener>,
    connections: HashMap<SocketAddr, Sender<Message<ServerMessage>>>,

    reciever: Receiver<(SocketAddr, Message<ClientMessage>)>,
    sender: Sender<(SocketAddr, Message<ClientMessage>)>,

    welcoming_reciever: Option<Receiver<(SocketAddr, Sender<Message<ServerMessage>>)>>,

    game_master: Option<GameMaster>,
    settings: ServerSettings,

    state: ServerState,
}

impl Server {
    pub async fn new() -> Server {
        // TODO! Should be read from a file
        let settings = ServerSettings {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6767),
            max_connections: 2,
            allow_spectators: false,
        };

        let (sender, reciever) =
            tokio::sync::mpsc::channel::<(SocketAddr, Message<ClientMessage>)>(1024); // random
        // number

        Server {
            listener: Some(
                TcpListener::bind(settings.addr)
                    .await
                    .expect("Could not start the server"),
            ), // TODO! Handle this nicely, as this will
            // crash gui app
            connections: HashMap::new(),
            sender,
            reciever,
            welcoming_reciever: None,
            game_master: None,
            settings,
            state: ServerState::default(),
        }
    }

    pub async fn start(&mut self) {
        let (welcoming_tx, welcoming_rx) =
            tokio::sync::mpsc::channel::<(SocketAddr, Sender<Message<ServerMessage>>)>(10);

        self.welcoming_reciever = Some(welcoming_rx);

        let listener = self
            .listener
            .take()
            .expect("Cannot start the server without tcp listener.");

        info!("Starting the server at: {:?}", listener.local_addr());

        let server_sender = self.sender.clone();

        tokio::spawn(async move {
            loop {
                if let Ok((socket, addr)) = listener.accept().await {
                    info!("New connection from: {:?}", addr);
                    let mut conn: Connection<ClientMessage, ServerMessage> =
                        Connection::new(ConnectionType::Client, server_sender.clone());

                    conn.delegate(socket, addr);

                    let sender = conn.get_sender();
                    let disconnect_sender = server_sender.clone();
                    let _ = welcoming_tx.send((addr, sender)).await;

                    tokio::spawn(async move {
                        conn.start_listening().await;

                        // TODO! Add waiting for reconnect

                        let disconnect_msg = Message::new(ClientMessage::ConnectionDead { addr });
                        let _ = disconnect_sender
                            .send((
                                addr,
                                disconnect_msg.expect(
                                    "Could not create disconnect msg. this shouldn't happen",
                                ),
                            ))
                            .await;
                    });
                }
            }
        });
    }

    /// Process incoming messages
    pub async fn update(&mut self) {
        loop {
            tokio::select! {
                new_client_opt = self.welcoming_reciever.as_mut().expect("Welcoming reciever was not set. Was the server started?").recv() => {
                    if let Some((addr, sender)) = new_client_opt {
                        if self.state.stage == ServerStage::Lobby && self.connections.keys().len() < self.settings.max_connections && !self.settings.allow_spectators {
                            self.connections.insert(addr, sender);
                            self.request_handshake(addr).await;
                        } else {
                            let reason = "There is currently game going and it doesn't allow spectators".to_string();
                            info!("Rejecting {}, because {}", addr, reason);

                            let msg = Message::new(ServerMessage::DeclineHandshake { reason: reason });

                            if let Ok(msg) = msg {
                            let _ = sender.send(msg).await; // we can't use send_message because we
                                                            // don't add addr to connections
                            } else {
                                warn!("Could not send reason of rejection. Server is acting like my ex...");
                                return;
                            }
                        }
                    } else {
                        warn!("Welcoming reciever closed!");
                        break;
                    }
                }

                msg_opt = self.reciever.recv() => {
                 if let Some(oldest_message) = msg_opt {
                            let (addr, msg) = oldest_message;

                            match msg.content {
                                ClientMessage::AnswerHandshake { player_name } => {
                                    self.process_handshake(addr, player_name);
                                }

                                ClientMessage::ConnectionDead { addr } => {
                                    warn!("Removing: {:?} from connection list", addr);
                                    // TODO! Should decrement ready_counter if player was ready. But
                                    // how to check that? Maybe create new struct that will hold
                                    // sender and ready and maybe some other data.
                                    self.connections.remove(&addr);
                                },

                                ClientMessage::SignalReadiness => {
                                    self.state.ready_count += 1;
                                    info!("{} signaled readiness. Currently ready: {}/{}", addr, self.state.ready_count, self.settings.max_connections);
                                    if self.state.ready_count == self.settings.max_connections {
                                        info!("All players are ready! Changing state to ServerStage::Game");
                                        self.state.stage = ServerStage::Game;
                                    }
                                },

                                ClientMessage::RequestCapture { capture_path: _ } => {
                                    self.process_capture();
                                }

                                ClientMessage::RequestMove { from: _, to: _ } => {
                                    self.process_move();
                                }

                                ClientMessage::TextMessage (content) => {
                                    info!("{} sent us a message: {}", addr, content);
                                }
                            }
                 } else {
                     warn!("Couldn't get latest message!");
                     break;
                 }
                }
            }
        }
    }

    /* ======= Processing messages :D helper functions ======== */

    fn process_handshake(&self, addr: SocketAddr, player_name: String) {
        info!("Client: {:?} sent us his name: {}", addr, player_name);
        // TODO! Do something with this name i guess
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

    fn remove_client(&mut self, addr: SocketAddr) {
        self.connections.remove(&addr);
    }
}
