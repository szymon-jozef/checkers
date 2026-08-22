use std::net::SocketAddr;

use log::{debug, error, info};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{
    logic::{board::pawn::CapturePath, math::position::Position},
    network::{
        connection::{Connection, ConnectionType},
        message::{
            ClientMessage::{self},
            Message, ServerMessage,
        },
    },
    settings::{client_settings::ClientSettings, general_settings::SettingsLike},
};

pub enum ClientCommands {
    SendCapture(CapturePath),
    SendMove { from: Position, to: Position },
    SendText(String),
    SendReady,
}

pub struct Client {
    conn_sender_outgoing: Sender<Message<ClientMessage>>,
    update_receiver: Option<Receiver<ServerMessage>>,
}

impl Client {
    /// Creates a new instance of clients, connects it to the server and starts update loop
    pub async fn new(settings: Option<ClientSettings>) -> Option<Client> {
        let Some(settings) = settings.or(Some(ClientSettings::new())) else {
            error!("Could not load client settings");
            return None;
        };

        let (conn_sender, mut conn_reciever) =
            mpsc::channel::<(SocketAddr, Message<ServerMessage>)>(1024);

        let mut conn: Connection<ServerMessage, ClientMessage> =
            Connection::new(ConnectionType::Server, conn_sender);

        let conn_sender_outgoing = conn.get_sender();

        let (update_sender, update_receiver) = mpsc::channel(1024);

        if !conn.connect_to_server(settings.server_url).await {
            error!("Could not connect with the server: {}", settings.server_url);
            return None;
        }

        tokio::spawn(async move {
            conn.start_listening().await;
        });

        let conn_sender_outgoing_clone = conn_sender_outgoing.clone();
        let settings_clone = settings.clone();
        let update_sender_clone = update_sender.clone();

        let (hanshake_tx, handshake_rx) = tokio::sync::oneshot::channel::<bool>();

        tokio::spawn(async move {
            let mut hanshake_tx = Some(hanshake_tx);

            loop {
                tokio::select! {
                    oldest_msg = conn_reciever.recv() => {
                        if let Some(oldest_msg) = oldest_msg {
                        let (addr, msg) = oldest_msg;
                        debug!("Got message from: {:?}", addr);

                        match msg.content {
                            ServerMessage::RequestHandshake => {
                                info!("Server requested handshake!");
                                let msg = Message::new(ClientMessage::AnswerHandshake {
                                    player_name: settings_clone.name.clone(),
                                });

                                let _ = conn_sender_outgoing_clone.send(msg.unwrap()).await;
                            }

                            ServerMessage::AcceptHandshake => {
                                info!("Server accepted us! Yay :D");
                                if let Some(hanshake_tx) = hanshake_tx.take() {
                                    let _ = hanshake_tx.send(true);
                                }
                            }

                            ServerMessage::DeclineHandshake { reason } => {
                                error!("Server declined connection: {}", reason);
                                if let Some(hanshake_tx) = hanshake_tx.take() {
                                    let _ = hanshake_tx.send(false);
                                }
                                break;
                            }

                            _ => { // Pass all messages that require player intervention to the
                                   // local client
                                let _ = update_sender_clone.send(msg.content).await;
                            }
                            }
                        } else {
                            error!("Connection broken!");
                            break;
                        }
                    }
                }
            }
        });

        let is_handshake_accepted: bool = match handshake_rx.await {
            Ok(is_accepted) => is_accepted,
            Err(e) => {
                error!("Error while receiving handshake: {}", e);
                return None;
            }
        };

        if is_handshake_accepted {
            Some(Client {
                conn_sender_outgoing,
                update_receiver: Some(update_receiver),
            })
        } else {
            None
        }
    }

    async fn send_message(&mut self, msg: Result<Message<ClientMessage>, postcard::Error>) {
        debug!("Sending message to the server!");
        let _ = match msg {
            Err(e) => {
                error!("There was an error while creating the message: {}", e);
                return;
            }
            Ok(msg) => {
                if let Err(e) = self.conn_sender_outgoing.send(msg).await {
                    error!("Error while sending a message to connection thread: {}", e);
                    return;
                }
            }
        };
    }

    pub async fn send_capture(&mut self, capture_path: CapturePath) {
        debug!("Requesting sending capture: {:?}", capture_path);

        let msg = Message::new(ClientMessage::RequestCapture { capture_path });
        self.send_message(msg).await;
    }

    pub async fn send_move(&mut self, from: Position, to: Position) {
        debug!("Requested sending move from: {} to: {}", from, to);
        let msg = Message::new(ClientMessage::RequestMove { from, to });
        self.send_message(msg).await;
    }

    pub async fn send_text_message(&mut self, content: String) {
        debug!("Sending text message with content: [{}]", content);
        let msg = Message::new(ClientMessage::TextMessage(content));
        self.send_message(msg).await;
    }

    pub async fn signal_readiness(&mut self) {
        debug!("Signaling readiness to the server...");
        let msg = Message::new(ClientMessage::SignalReadiness);
        self.send_message(msg).await;
    }

    pub async fn revoke_readiness(&mut self) {
        debug!("Revoking readiness and sending it to the server...");
        let msg = Message::new(ClientMessage::SignalUnreadiness);
        self.send_message(msg).await;
    }

    pub fn get_update_receiver(&mut self) -> Option<Receiver<ServerMessage>> {
        self.update_receiver.take()
    }
}
