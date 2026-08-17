use std::{
    fmt::Display,
    io::{self, Write},
};

use checkers::network::{
    client::{Client, ClientData},
    network_identity::NetworkIdentity,
};
use log::{debug, error, info, warn};
use tokio::sync::mpsc;
use uuid::Uuid;

const QUIT_COMMAND: &str = "/quit";
const HELP_COMMAND: &str = "/help";
const SEND_COMMAND: &str = "/send";
const READY_COMMAND: &str = "/ready";

enum CliCommands {
    Quit,
    Help,
    Send,
    Ready,
}

#[derive(Debug, PartialEq)]
struct UnknownCommandError;

impl Display for UnknownCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown command!")
    }
}

impl TryFrom<&str> for CliCommands {
    type Error = UnknownCommandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            QUIT_COMMAND => Ok(CliCommands::Quit),
            HELP_COMMAND => Ok(CliCommands::Help),
            SEND_COMMAND => Ok(CliCommands::Send),
            READY_COMMAND => Ok(CliCommands::Ready),
            _ => Err(UnknownCommandError),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let Some(mut client) = Client::new().await else {
        println!("Not worky, ending");
        return Ok(());
    };

    let (client_sender, mut client_receiver) = mpsc::channel(1024);

    client.set_update_sender(client_sender);
    client.update();

    let (stdin_sender, mut stdin_receiver) = mpsc::channel(1024);

    let mut indentity: Option<NetworkIdentity> = None;
    let mut is_my_turn: bool = false;

    tokio::task::spawn_blocking(move || {
        let stdin = std::io::stdin();

        loop {
            let mut buffer = String::new();
            if stdin.read_line(&mut buffer).is_ok() {
                if stdin_sender.blocking_send(buffer).is_err() {
                    break;
                }
            }
        }
    });

    let help_msg: String = format!(
        "\n┌────Help────────────────────────────
│\t {} - quit                        
│\t {} - show this message             
│\t {} - message the server you're ready
│\t {} - send text message to the server
└───────────────────────────────────────
        ",
        QUIT_COMMAND, HELP_COMMAND, READY_COMMAND, SEND_COMMAND,
    );

    info!("───────Type /help for help───────");
    loop {
        tokio::select! {
        msg = client_receiver.recv() => {
            match msg {
                Some(ClientData::GameStart {
                    identity,
                    board_view,
                }) => {
                    info!(
                        "Got new identity: {} and board_view. Game is about to start!",
                        identity
                    );
                    println!("{}", board_view.to_string(&identity.id));
                    indentity = Some(identity);
                }

                Some(ClientData::BoardView(view)) => {
                    info!("New board state:\n{}", view.to_string(&indentity.as_ref().unwrap().id));

                }

                Some(ClientData::CurrentTurn(current_turn)) => {
                    is_my_turn = current_turn == indentity.as_ref().expect("Server sent us new current turn without telling us who we are!").id;
                    if is_my_turn {
                        info!("It's our turn!");
                    } else {
                        info!("It's the turn of our enemy!");
                    }
                }

                Some(ClientData::AvailableCaptures(capture_paths)) => {
                    todo!()
                }

                Some(ClientData::AvailableMoves(move_paths)) => todo!(),

                Some(ClientData::TextMessage(content)) => {
                    info!("Got message from the server: {}", content);
                }

                Some(ClientData::GameEnd(game_result)) => {
                    info!("Game has ended with the result: {:?}", game_result);
                }

                None => {
                    warn!("Connection broken! Stopping");
                    break Ok(());
                }
            }
            }

        buffer_opt = stdin_receiver.recv() => {
            if let Some(mut buffer) = buffer_opt {
                buffer.pop(); // remove \n

                if !buffer.is_empty() {
                    let words: Vec<&str> = buffer.split_whitespace().collect();
                    let cmd = words[0];
                    let mut args: String = words[1..].join(" ");
                    args = args.trim().to_string();

                    match CliCommands::try_from(cmd) {
                        Ok(cmd) => match cmd {
                            CliCommands::Quit => {
                                info!("Bye bye");
                                std::process::exit(0);
                            }
                            CliCommands::Help => {
                                info!("{}", help_msg);
                            }
                            CliCommands::Ready => {
                                client.signal_readiness().await;
                            }
                            CliCommands::Send => {
                                client.send_text_message(args).await;
                            }
                        },
                        Err(e) => error!("{}", e),
                    }
                }
                print!("> ");
                io::stdout().flush()?;
            }
        }
        }
    }
}
