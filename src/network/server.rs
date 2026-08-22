use log::{debug, error, info, warn};
use std::{collections::HashMap, net::SocketAddr};
use tokio::{
    net::TcpListener,
    sync::mpsc::{Receiver, Sender},
};
use uuid::Uuid;

use crate::{
    logic::{
        board::pawn::CapturePath,
        game_master::{GameMaster, GameResult},
        math::position::Position,
    },
    network::{
        connection::{Connection, ConnectionType},
        message::{
            ClientMessage::{self},
            Message, ServerMessage,
        },
        network_identity::NetworkServerIdentity,
    },
    settings::{general_settings::SettingsLike, server_settings::ServerSettings},
};

// TODO! Maybe move this somewhere more global and share with `board` struct
const MAX_PLAYABLE_CONNECTIONS: usize = 2; // DON'T EVER CHANGE THIS AS GAME OF CHECKERS HAS ONLY TWO PLAYERS, U
// UNDERSTAND????
//
// There will be a spectator mode, hence the name __PLAYABLE__ connection. You will be able to have
// more connections, but they won't be able to play

#[derive(Debug, PartialEq, Default)]
pub enum ServerStage {
    #[default]
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
    connections: HashMap<SocketAddr, NetworkServerIdentity>,

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
        let settings = ServerSettings::new();

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
            connections: HashMap::with_capacity(2),
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
                        if self.state.stage == ServerStage::Lobby && self.connections.keys().len() < MAX_PLAYABLE_CONNECTIONS && !self.settings.allow_spectators {
                            let identity = NetworkServerIdentity::new(sender);
                            self.connections.insert(addr, identity);
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
                                    self.process_handshake(addr, player_name).await;
                                }

                                ClientMessage::ConnectionDead { addr } => {
                                    self.remove_client(addr);
                                },

                                ClientMessage::SignalReadiness => {
                                    self.handle_readiness(addr).await
                                }

                                ClientMessage::SignalUnreadiness => {
                                    self.handle_unreadiness(addr).await;
                                }

                                ClientMessage::RequestCapture { capture_path } => {
                                    self.process_capture(capture_path).await;
                                }

                                ClientMessage::RequestMove { from, to } => {
                                    self.process_move(from, to).await;
                                }

                                ClientMessage::TextMessage (content) => {
                                    info!("{} sent us a message: {}", addr, content);
                                    let sender = self.connections[&addr].identity.name.clone();
                                    self.broadcast_text_message(sender, content).await;
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

    async fn start_game(&mut self) {
        info!("Starting the game!");
        let mut names: Vec<String> = vec![];
        for networkidentity in self.connections.values() {
            if networkidentity.identity.is_ready {
                names.push(networkidentity.identity.name.clone());
            }
        }

        if names.len() != MAX_PLAYABLE_CONNECTIONS {
            error!("Tried starting the game, but could not gather enough ready player names");
            return;
        }

        let game_master = GameMaster::new(&names[0], &names[1]);

        let names_and_ids: Vec<(String, Uuid)> = game_master.get_players_names_and_ids();

        let mut msgs: Vec<(SocketAddr, Result<Message<ServerMessage>, postcard::Error>)> = vec![];

        for (name, id) in names_and_ids {
            for (addr, networkidentity) in self.connections.iter_mut() {
                if networkidentity.identity.name == name {
                    networkidentity.identity.id = id;

                    msgs.push((
                        *addr,
                        Message::new(ServerMessage::GameStart {
                            identity: networkidentity.identity.clone(),
                        }),
                    ));
                }
            }
        }

        for (addr, msg) in msgs {
            self.send_message(addr, msg).await;
        }

        self.game_master = Some(game_master);
        self.on_player_action().await;
    }

    /* ======= Processing messages :D helper functions ======== */

    async fn process_handshake(&mut self, addr: SocketAddr, player_name: String) {
        info!("Client: {:?} sent us his name: {}", addr, player_name);
        self.send_message(addr, Message::new(ServerMessage::AcceptHandshake))
            .await;

        let fixed_name: String;

        if self
            .connections
            .values()
            .any(|v| v.identity.name == player_name)
        {
            fixed_name = format!("{}(2)", player_name);
            warn!(
                "Player has the same name as the other player so changing it to {}",
                fixed_name
            );
        } else {
            fixed_name = player_name;
        }

        let Some(identity) = self.connections.get_mut(&addr) else {
            error!("Could not get network identity while processing the handshake!");
            return;
        };

        identity.identity.name = fixed_name;
        identity.is_handshaken = true;
    }

    async fn process_capture(&mut self, capture_path: CapturePath) {
        if let Some(gm) = self.game_master.as_mut() {
            if gm.capture(&capture_path) {
                debug!("Capture valid!");
                self.on_player_action().await;
            } else {
                debug!("Invalid capture!");
            }
        }
    }

    async fn process_move(&mut self, from: Position, to: Position) {
        if let Some(gm) = self.game_master.as_mut() {
            if gm.move_pawn(from, to) {
                debug!("Move was valid!");
                self.on_player_action().await;
            } else {
                debug!("Move was invalid!");
            }
        }
    }

    async fn request_handshake(&mut self, addr: SocketAddr) {
        let msg = Message::new(ServerMessage::RequestHandshake);
        self.send_message(addr, msg).await;
    }

    async fn handle_readiness(&mut self, addr: SocketAddr) {
        if !self.connections[&addr].is_handshaken {
            warn!(
                "Connection: {} tried sending readiness, but it have not shaken hands. Ignoring...",
                addr
            );
            return;
        }

        self.state.ready_count += 1;
        info!(
            "{} signaled readiness. Currently ready: {}/{}",
            addr, self.state.ready_count, MAX_PLAYABLE_CONNECTIONS
        );
        self.connections.get_mut(&addr).expect("Couldn't get the identity of the ready player. I guess this should never happen so i'm panicking like a little bitch").identity.is_ready = true; // TODO! This sometimes happens for some reason (when there is a game going, both player leave and someone tries to join)

        if self.state.ready_count == MAX_PLAYABLE_CONNECTIONS {
            info!("All players are ready! Changing state to ServerStage::Game");
            self.state.stage = ServerStage::Game;
            self.start_game().await;
        }
    }

    async fn handle_unreadiness(&mut self, addr: SocketAddr) {
        if !self.connections[&addr].is_handshaken {
            warn!(
                "Connection: {} tried revoking readiness, but hands were not shaken. Ignoring....",
                addr
            );
            return;
        }

        if self.state.stage != ServerStage::Lobby {
            warn!(
                "Player {} tried revoking readiness, but game already started. Ignoring...",
                addr
            );
            return;
        }

        self.state.ready_count -= 1;

        self.connections
            .get_mut(&addr)
            .expect("Could not get the indentity of the player that wanted to revoke readinessl")
            .identity
            .is_ready = false;

        info!(
            "Player {} revoked his readiness. Currently ready: {}/{}",
            addr, self.state.ready_count, MAX_PLAYABLE_CONNECTIONS
        );
    }

    /* === Sending messages === */

    ///  Should be used after every players move
    async fn on_player_action(&mut self) {
        if self.state.stage == ServerStage::Game {
            self.broadcast_board_view().await;
            self.broadcast_current_turn().await;

            let Some(gm) = &self.game_master else {
                error!("Game master not set while player made an action!");
                return;
            };

            let captures = gm.get_current_player_captures();
            let moves = gm.get_current_player_moves();
            let current_player = gm.get_current_turn();

            if let Some(result) = gm.get_game_result() {
                info!("Game ended with result: {:?}", result);
                self.handle_game_end(result).await;
                return;
            }

            if !captures.is_empty() {
                debug!("Current captures: {:?}", captures);
                self.send_player_captures(current_player, captures).await;
                return;
            }

            if !moves.is_empty() {
                debug!("Current moves: {:?}", moves);
                self.send_player_moves(current_player, moves).await;
                return;
            }

            panic!("on_player_action should never get here");
        }
    }

    async fn send_player_captures(&mut self, current_player: Uuid, captures: Vec<CapturePath>) {
        let Some(addr) = self.get_client_by_id(current_player) else {
            error!("Could not get player connection, while sending him his captures!");
            return;
        };

        let msg = Message::new(ServerMessage::AvailableCaptures { captures });
        self.send_message(*addr, msg).await;
    }

    async fn send_player_moves(
        &mut self,
        current_player: Uuid,
        moves: Vec<crate::logic::board::pawn::MovePath>,
    ) {
        let Some(addr) = self.get_client_by_id(current_player) else {
            error!("Could not get player connection, while sending him his moves!");
            return;
        };

        let msg = Message::new(ServerMessage::AvailableMoves { moves });
        self.send_message(*addr, msg).await;
    }

    async fn handle_game_end(&mut self, result: GameResult) {
        let msg = Message::new(ServerMessage::GameEnd { result });
        self.broadcast_message(msg).await;
        self.state.stage = ServerStage::End;
    }

    async fn broadcast_board_view(&mut self) {
        if let Some(gm) = &self.game_master {
            let msg = Message::new(ServerMessage::BroadcastBoardState {
                board: gm.get_board_view(),
            });
            self.broadcast_message(msg).await;
        } else {
            error!("Cannot broadcast board view as game manager is not set!");
        }
    }

    async fn broadcast_current_turn(&mut self) {
        if let Some(gm) = &self.game_master {
            let msg = Message::new(ServerMessage::BroadcastCurrentTurn {
                active_player: gm.get_current_turn(),
            });
            self.broadcast_message(msg).await;
        } else {
            error!("Cannot broadcast current turn as game master doesn't exists!!!!!!!!!");
        }
    }

    async fn broadcast_text_message(&mut self, sender: String, content: String) {
        let msg = Message::new(ServerMessage::BroadCastTextMessage { sender, content });
        self.broadcast_message(msg).await;
    }

    /* === Helper methods === */

    async fn send_message(
        &mut self,
        addr: SocketAddr,
        msg: Result<Message<ServerMessage>, postcard::Error>,
    ) {
        let conn_opt = self.connections.get_mut(&addr);

        let Some(identity) = conn_opt else {
            error!("Could not get identity from connections list");
            return;
        };

        let _ = match msg {
            Err(e) => {
                error!("There was an error while creating message: {}", e);
                return;
            }
            Ok(msg) => identity.sender.send(msg).await,
        };
    }

    async fn broadcast_message(&mut self, msg: Result<Message<ServerMessage>, postcard::Error>) {
        match msg {
            Ok(msg) => {
                for identity in self.connections.values_mut() {
                    let _ = identity.sender.send(msg.clone()).await;
                }
            }
            Err(e) => {
                error!("Error while broadcasting the message: {}", e);
                return;
            }
        }
    }

    fn get_client_by_id(&self, id: Uuid) -> Option<&SocketAddr> {
        self.connections
            .iter()
            .find(|(_, indentity)| indentity.identity.id == id)
            .map(|(addr, _)| addr)
    }

    fn remove_client(&mut self, addr: SocketAddr) {
        warn!("Removing: {:?} from connection list", addr);
        let is_ready = &mut self.connections.get_mut(&addr).expect("Couldn't get the identity of disconnected player. I don't think i should panic but i don't care fuck you").identity.is_ready;

        if *is_ready {
            *is_ready = false;
            self.state.ready_count -= 1;
            info!(
                "Currently {}/{} players are ready",
                self.state.ready_count, MAX_PLAYABLE_CONNECTIONS
            );
        }
        self.connections.remove(&addr);
    }
}
