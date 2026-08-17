use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use log::{debug, error, info, warn};
use tokio::sync::mpsc::{self, Receiver, Sender};
use uuid::Uuid;

use crate::{
    logic::{
        board::{
            board_view::BoardView,
            pawn::{CapturePath, MovePath},
        },
        game_master::GameResult,
        math::position::Position,
    },
    network::{
        connection::{Connection, ConnectionType},
        message::{
            ClientMessage::{self, SignalReadiness, TextMessage},
            Message, ServerMessage,
        },
        network_identity::NetworkIdentity,
    },
};

#[derive(Clone)]
pub struct ClientSettings {
    server_url: SocketAddr,
    name: String,
}

pub enum ClientCommands {
    SendCapture(CapturePath),
    SendMove { from: Position, to: Position },
    SendText(String),
    SendReady,
}

/// Data that network client gives to normal client. I have no better idea for this name, hence this
/// comment
pub enum ClientData {
    GameStart { identity: NetworkIdentity },
    GameEnd(GameResult),

    AvailableCaptures(Vec<CapturePath>),
    AvailableMoves(Vec<MovePath>),

    BoardView(BoardView),
    CurrentTurn(Uuid),

    TextMessage { sender: String, content: String },
}

pub struct Client {
    conn_reciever_incoming: Option<Receiver<(SocketAddr, Message<ServerMessage>)>>,
    conn_sender_outgoing: Sender<Message<ClientMessage>>,
    settings: ClientSettings,

    commands_sender: Option<Sender<ClientCommands>>,
    commands_receiver: Option<Receiver<ClientCommands>>,

    update_sender: Option<Sender<ClientData>>,
}

impl Client {
    pub async fn new() -> Option<Client> {
        let settings = ClientSettings {
            server_url: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6767),
            name: "Morbius".into(),
        };

        let (conn_sender, conn_reciever) =
            mpsc::channel::<(SocketAddr, Message<ServerMessage>)>(1024);
        let mut conn: Connection<ServerMessage, ClientMessage> =
            Connection::new(ConnectionType::Server, conn_sender);
        let conn_sender_outgoing = conn.get_sender();

        let (commands_sender, commands_receiver) = mpsc::channel(1024);

        if !conn.connect_to_server(settings.server_url).await {
            error!("Could not connect with the server: {}", settings.server_url);
            return None;
        }

        tokio::spawn(async move {
            conn.start_listening().await;
        });

        Some(Client {
            conn_reciever_incoming: Some(conn_reciever),
            conn_sender_outgoing,

            settings,

            commands_receiver: Some(commands_receiver),
            commands_sender: Some(commands_sender),

            update_sender: None,
        })
    }

    pub fn get_commands_sender(&mut self) -> Option<Sender<ClientCommands>> {
        if self.commands_sender.is_none() {
            warn!("Tried taking commands sender a second time! This shouldn't happen");
        }
        self.commands_sender.take()
    }

    pub fn set_update_sender(&mut self, sender: Sender<ClientData>) {
        if self.update_sender.is_some() {
            warn!("Cannot set update sender a second time!");
            return;
        }

        self.update_sender = Some(sender);
    }

    pub fn update(&mut self) {
        let Some(mut conn_reciever_incoming) = self.conn_reciever_incoming.take() else {
            error!("Tried updating, but there's no conn_reciever of incoming messages!");
            return;
        };

        let conn_sender = self.conn_sender_outgoing.clone();

        let Some(update_sender) = self.update_sender.take() else {
            error!("Update sender is missing. Cannot update!");
            return;
        };

        let Some(mut commands_receiver) = self.commands_receiver.take() else {
            error!("Commands receiver is missing! Cannot update.");
            return;
        };

        let settings = self.settings.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    oldest_msg = conn_reciever_incoming.recv() => {
                        if let Some(oldest_msg) = oldest_msg {
                        let (addr, msg) = oldest_msg;
                        debug!("Got message from: {:?}", addr);

                        match msg.content {
                            ServerMessage::RequestHandshake => {
                                info!("Server requested handshake!");
                                let msg = Message::new(ClientMessage::AnswerHandshake {
                                    player_name: settings.name.clone(),
                                });

                                let _ = conn_sender.send(msg.unwrap()).await;
                            }

                            ServerMessage::AcceptHandshake => {
                                info!("Server accepted us! Yay :D");
                            }

                            ServerMessage::DeclineHandshake { reason } => {
                                error!("Server declined connection: {}", reason);
                                break;
                            }

                            ServerMessage::GameStart {
                                identity,
                            } => {
                                debug!("Got GameStart message! Identity: {}", identity);
                                let _ = update_sender.send(ClientData::GameStart { identity }).await;
                            }

                            ServerMessage::BroadcastCurrentTurn { active_player } => {
                                let _ = update_sender.send(ClientData::CurrentTurn(active_player)).await;
                            },
                            ServerMessage::BroadcastBoardState { board } => {
                                let _ = update_sender.send(ClientData::BoardView(board)).await;
                            },

                            ServerMessage::BroadCastTextMessage {sender, content} => {
                                let _ = update_sender.send(ClientData::TextMessage {sender, content}).await;
                            }


                            ServerMessage::AvailableCaptures { captures } => todo!(),
                            ServerMessage::AvailableMoves { moves } => todo!(),
                            ServerMessage::GameEnd { result } => todo!(),
                            }
                        } else {
                            error!("Connection broken!");
                            break;
                        }
                    }

                    oldest_cmd = commands_receiver.recv() => {
                        if let Some(oldest_cmd) = oldest_cmd {
                            match oldest_cmd {
                                ClientCommands::SendCapture(capture_path) => todo!(),

                                ClientCommands::SendMove { from, to } => todo!(),

                                ClientCommands::SendText(content) => {
                                    debug!("Seding text message: {}", content);
                                    let msg = Message::new(TextMessage(content));
                                    send_message(msg, &conn_sender).await;
                                },

                                ClientCommands::SendReady => {
                                    debug!("Signaling readiness!");
                                    let msg = Message::new(SignalReadiness);
                                    send_message(msg, &conn_sender).await;
                                },
                            }
                        } else {
                            error!("Couldn't decipher the command!");
                            break;
                        }
                    }

                }
            }
        });
    }

    pub async fn send_text_message(&mut self, content: String) {
        if let Some(sender) = &self.commands_sender {
            let _ = sender.send(ClientCommands::SendText(content)).await;
        } else {
            error!("Command sender not set!");
        }
    }

    pub async fn signal_readiness(&mut self) {
        if let Some(sender) = &self.commands_sender {
            let _ = sender.send(ClientCommands::SendReady).await;
        } else {
            error!("Command sender not set!");
        }
    }
}

async fn send_message(
    msg: Result<Message<ClientMessage>, postcard::Error>,
    sender: &Sender<Message<ClientMessage>>,
) {
    debug!("Sending message to the server!");
    let _ = match msg {
        Err(e) => {
            error!("There was an error while creating the message: {}", e);
            return;
        }
        Ok(msg) => {
            if let Err(e) = sender.send(msg).await {
                error!("Error while sending a message to connection thread: {}", e);
                return;
            }
        }
    };
}
