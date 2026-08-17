use std::{
    fmt::Display,
    io::{self, Write},
};

use checkers::network::client::Client;
use log::{debug, error, info};
use tokio::sync::mpsc;

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

    let help_msg: String = format!(
        "\n┌────Help────
│\t {} - quit
│\t {} - show this message
│\t {} - message the server you're ready
│\t {} - send text message to the server
        ",
        QUIT_COMMAND, HELP_COMMAND, READY_COMMAND, SEND_COMMAND,
    );

    info!("====== Type /help for help ========");
    let mut buffer = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut buffer)?;
        buffer.pop(); // remove \n

        match client_receiver.try_recv() {
            Ok(checkers::network::client::ClientData::GameStart {
                identity,
                board_view,
            }) => {
                info!(
                    "Got new identity: {} and board_view. Game is about to start!",
                    identity
                );
                println!("{}", board_view.to_string(identity.id));
            }

            Ok(checkers::network::client::ClientData::AvailableCaptures(capture_paths)) => {
                todo!()
            }

            Ok(checkers::network::client::ClientData::AvailableMoves(move_paths)) => todo!(),

            Ok(checkers::network::client::ClientData::TextMessage(content)) => {
                info!("Got message from the server: {}", content);
            }

            Ok(checkers::network::client::ClientData::GameEnd(game_result)) => {
                info!("Game has ended with the result: {:?}", game_result);
            }

            _ => {}
        }

        if buffer.is_empty() {
            buffer.clear();
            continue;
        }

        let words: Vec<&str> = buffer.split_whitespace().collect();
        let cmd = words[0];
        let mut args: String = words[1..].join(" ");
        args = args.trim().to_string();

        match CliCommands::try_from(cmd) {
            Ok(cmd) => match cmd {
                CliCommands::Quit => {
                    info!("Bye bye");
                    break;
                }
                CliCommands::Help => {
                    info!("{}", help_msg);
                    buffer.clear();
                }
                CliCommands::Ready => {
                    client.signal_readiness().await;
                    buffer.clear();
                }
                CliCommands::Send => {
                    client.send_text_message(args).await;
                    buffer.clear();
                }
            },
            Err(e) => error!("{}", e),
        }
        buffer.clear();
    }

    return Ok(());
}
