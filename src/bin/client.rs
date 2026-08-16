use std::{
    fmt::Display,
    io::{self, Write},
    time::Duration,
};

use checkers::network::client::Client;
use env_logger::TimestampPrecision::Seconds;
use log::{error, info};

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

    let help_msg: String = format!(
        "\n┌────Help────
│\t {} - quit
│\t {} - show this message
│\t {} - message the server you're ready
│\t {} - send text message to the server
        ",
        QUIT_COMMAND, HELP_COMMAND, READY_COMMAND, SEND_COMMAND,
    );

    client.update();

    info!("====== Type /help for help ========");
    let mut buffer = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut buffer)?;
        buffer.pop(); // remove \n

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
