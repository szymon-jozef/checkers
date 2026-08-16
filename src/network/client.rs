use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use log::{debug, error, info};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::network::{
    connection::{Connection, ConnectionType},
    message::{
        ClientMessage::{self, TextMessage},
        Message, ServerMessage,
    },
};

#[derive(Clone)]
pub struct ClientSettings {
    server_url: SocketAddr,
    name: String,
}

pub struct Client {
    conn_reciever_incoming: Option<Receiver<(SocketAddr, Message<ServerMessage>)>>,
    conn_sender_outgoing: Sender<Message<ClientMessage>>,
    settings: ClientSettings,
}

impl Client {
    pub async fn new() -> Option<Client> {
        let settings = ClientSettings {
            server_url: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6767),
            name: "Szymon".into(),
        };

        let (conn_sender, conn_reciever) =
            mpsc::channel::<(SocketAddr, Message<ServerMessage>)>(1024);

        let mut conn: Connection<ServerMessage, ClientMessage> =
            Connection::new(ConnectionType::Server, conn_sender);

        let conn_sender_outgoing = conn.get_sender();

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
        })
    }

    pub fn update(&mut self) {
        let Some(mut conn_reciever_incoming) = self.conn_reciever_incoming.take() else {
            error!("Tried updating, but there's no conn_reciever of incoming messages!");
            return;
        };

        let sender = self.conn_sender_outgoing.clone();

        let settings = self.settings.clone();

        tokio::spawn(async move {
            loop {
                if let Some(oldest_msg) = conn_reciever_incoming.recv().await {
                    let (addr, msg) = oldest_msg;
                    info!("Got message from: {:?}", addr);

                    match msg.content {
                        ServerMessage::RequestHandshake => {
                            info!("Server requested handshake!");
                            let msg = Message::new(ClientMessage::AnswerHandshake {
                                player_name: settings.name.clone(),
                            });

                            let _ = sender.send(msg.unwrap()).await;
                        }
                        ServerMessage::AcceptHandshake { player_id } => todo!(),
                        ServerMessage::DeclineHandshake { reason } => todo!(),
                        ServerMessage::AvailableCaptures { captures } => todo!(),
                        ServerMessage::AvailableMoves { moves } => todo!(),
                        ServerMessage::BroadcastBoardState { board } => todo!(),
                        ServerMessage::BroadcastCurrentTurn { active_player } => todo!(),
                        ServerMessage::GameEnd { result } => todo!(),
                    }
                }
            }
        });
    }

    pub async fn send_text_message(&mut self, content: String) {
        let msg = Message::new(TextMessage(content));
        self.send_message(msg).await;
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
}
