use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;

use log::{debug, error, warn};
use postcard::from_bytes;
use tokio::{net::TcpStream, sync::mpsc::Sender};

use tokio::io::AsyncReadExt;

use crate::network::message::{Message, MessageLike};

/// Enum defines __TO WHAT__ connection is connecting
pub enum ConnectionType {
    Server,
    Client,
}

pub struct Connection<Inbound, Outbound>
where
    Inbound: MessageLike,
{
    conn_type: ConnectionType,
    sender: Sender<Message<Inbound>>,
    tcp: Option<TcpStream>,
    outbound_type: PhantomData<Outbound>,
}

impl<Inbound, Outbound> Connection<Inbound, Outbound>
where
    Inbound: MessageLike,
    Outbound: MessageLike,
{
    pub fn new(
        conn_type: ConnectionType,
        sender: Sender<Message<Inbound>>,
    ) -> Connection<Inbound, Outbound> {
        Connection {
            conn_type,
            sender,
            tcp: None,
            outbound_type: PhantomData,
        }
    }

    pub async fn connect_to_server(&mut self, url: SocketAddr) -> bool {
        match self.conn_type {
            ConnectionType::Server => {
                self.tcp = match TcpStream::connect(url).await {
                    Ok(conn) => Some(conn),
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

    pub async fn send(&mut self, msg: Message<Outbound>) {
        todo!();
    }

    pub async fn start_listening(&mut self) {
        loop {
            let result = self.read_header().await;

            if let Err(e) = result {
                error!(
                    "Error while reading data from connection with: {} {}. Connection is broken. Disconnecting...",
                    self.tcp.as_ref().unwrap().peer_addr().unwrap(),
                    e
                );
                return;
            }
        }
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
                        if let Err(e) = self.sender.send(msg).await {
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
