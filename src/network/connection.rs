use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;

use log::{debug, error, info};
use tokio::sync::mpsc::Receiver;
use tokio::{net::TcpStream, sync::mpsc::Sender};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::network::message::{Message, MessageLike};

/// Enum defines __TO WHAT__ connection is connecting
#[derive(Debug)]
pub enum ConnectionType {
    Server,
    Client,
}

#[derive(Debug)]
pub struct Connection<Inbound, Outbound>
where
    Inbound: MessageLike,
    Outbound: MessageLike,
{
    conn_type: ConnectionType,
    sender: Sender<(SocketAddr, Message<Inbound>)>,
    reciever: Option<Receiver<Message<Outbound>>>,
    tcp: Option<TcpStream>,
    peer: Option<SocketAddr>,
    outbound_type: PhantomData<Outbound>,
}

impl<Inbound, Outbound> Connection<Inbound, Outbound>
where
    Inbound: MessageLike,
    Outbound: MessageLike,
{
    pub fn new(
        conn_type: ConnectionType,
        sender: Sender<(SocketAddr, Message<Inbound>)>,
    ) -> Connection<Inbound, Outbound> {
        Connection {
            conn_type,
            sender,
            reciever: None,
            tcp: None,
            peer: None,
            outbound_type: PhantomData,
        }
    }

    /// Meant for clients
    pub async fn connect_to_server(&mut self, url: SocketAddr) -> bool {
        match self.conn_type {
            ConnectionType::Server => {
                self.tcp = match TcpStream::connect(url).await {
                    Ok(conn) => {
                        self.peer = Some(url);
                        Some(conn)
                    }
                    Err(e) => {
                        error!("Error while connecting to the server: {}", e);
                        None
                    }
                };

                self.tcp.is_some()
            }
            ConnectionType::Client => false, // only client can connect to the server
        }
    }

    /// Meant for server
    pub fn delegate(&mut self, tcp: TcpStream, client_addr: SocketAddr) -> bool {
        match self.conn_type {
            ConnectionType::Server => false, // only server can delegate
            ConnectionType::Client => {
                self.peer = Some(client_addr);
                self.tcp = Some(tcp);
                true
            }
        }
    }

    pub fn get_sender(&mut self) -> Sender<Message<Outbound>> {
        let (sender, reciever) = tokio::sync::mpsc::channel(1024);
        self.reciever = Some(reciever);
        sender
    }

    async fn send(&mut self, msg: Message<Outbound>) {
        let size = msg.size;
        let content = msg.content;

        let Some(tcp) = &mut self.tcp else {
            error!("Tcp connection is not established! Cannot send");
            return;
        };

        if let Err(e) = tcp.write_all(&size.to_be_bytes()).await {
            error!("Error while sending header to: {:?}\n{}", self.peer, e);
            return;
        }

        let Ok(msg) = content.to_bits() else {
            error!("Cannot change message into bytes while sending!");
            return;
        };

        if let Err(e) = tcp.write_all(&msg).await {
            error!("Error while sending content to: {:?}\n{}", self.peer, e);
            return;
        };
    }

    pub async fn start_listening(&mut self) {
        let mut receiver = self
            .reciever
            .take()
            .expect("Tried listening, but get_sender wasn't called");

        loop {
            tokio::select! {
                result = self.read_header() => {
                    if let Err(e) = result {
                        error!(
                            "Connection broken with {:?}: {}. Disconnecting...",
                            self.peer, e
                        );
                        break;
                    }
                },

                msg_opt = receiver.recv() => {
                    match msg_opt {
                        Some(msg) => {
                            self.send(msg).await;
                        },
                        None => {
                            info!("Remote closed the channel for {:?}. Disconnecting...", self.peer);
                            break;
                        }
                    }
                }
            }
        }

        self.reciever = Some(receiver);
    }

    async fn read_header(&mut self) -> Result<(), io::Error> {
        let mut buff = [0u8; 4];

        let tcp = self.tcp.as_mut().ok_or(io::ErrorKind::NotConnected)?;

        tcp.read_exact(&mut buff).await?;

        let size_to_read: u32 = u32::from_be_bytes(buff);

        self.read_content(size_to_read).await
    }

    async fn read_content(&mut self, size_to_read: u32) -> Result<(), io::Error> {
        let mut buff: Vec<u8> = vec![0; size_to_read as usize];

        let Some(tcp) = &mut self.tcp else {
            error!(
                "Tried reading content, but tcp connection is not established! This should __never__ happen"
            );
            panic!();
        };

        let bytes_read = tcp.read_exact(&mut buff).await?;
        debug!("Read {} bytes during read_content", bytes_read);

        let msg_result: Result<Inbound, postcard::Error> = MessageLike::from_bits(&buff);

        match msg_result {
            Ok(msg_content) => {
                let new_msg = Message::new(msg_content);

                match new_msg {
                    Ok(msg) => {
                        if let Err(e) = self
                            .sender
                            .send((self.peer.expect("Couldn't get peer addr"), msg))
                            .await
                        {
                            error!("Erro while sending message to main thread: {}", e);
                        }
                        return Ok(());
                    }
                    Err(e) => {
                        error!(
                            "Error while preparing message in read_content for sender: {}",
                            e
                        );
                        return Ok(());
                    }
                }
            }
            Err(e) => {
                error!("Error while getting the message from bits: {}", e);
                return Ok(());
            }
        }
    }
}
