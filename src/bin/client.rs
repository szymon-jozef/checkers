use std::{
    fmt::Display,
    io::{self, Write},
};

use checkers::{
    logic::board::pawn::{CapturePath, MovePath},
    network::{client::Client, message::ServerMessage, network_identity::NetworkIdentity},
    super_advanced_ai::{Bot, BotDificulty},
};
use log::{error, warn};
use tokio::sync::mpsc;

const QUIT_COMMAND: &str = "/quit";
const HELP_COMMAND: &str = "/help";
const SEND_COMMAND: &str = "/send";
const READY_COMMAND: &str = "/ready";
const UNREADY_COMMAND: &str = "/unready";
const SINGLEPLAYER_COMMAND: &str = "/single";

const CAPTURE_COMMAND: &str = "/capture";
const MOVE_COMMAND: &str = "/move";

enum CliCommands {
    Quit,
    Help,
    Send,

    Ready,
    Unready,

    Single,

    Capture,
    Move,
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
            UNREADY_COMMAND => Ok(CliCommands::Unready),
            SINGLEPLAYER_COMMAND => Ok(CliCommands::Single),

            CAPTURE_COMMAND => Ok(CliCommands::Capture),
            MOVE_COMMAND => Ok(CliCommands::Move),
            _ => Err(UnknownCommandError),
        }
    }
}

struct ClientContext {
    pub identity: Option<NetworkIdentity>,
    pub is_my_turn: bool,

    pub available_captures: Option<Vec<CapturePath>>,
    pub available_moves: Option<Vec<MovePath>>,
}

impl Default for ClientContext {
    fn default() -> Self {
        Self {
            identity: None,
            is_my_turn: false,

            available_captures: None,
            available_moves: None,
        }
    }
}

async fn run_bot() {
    let bot = Bot::new(BotDificulty::Easy).await;
    bot.game_loop().await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let Some(mut client) = Client::new(None).await else {
        println!("Not worky, ending");
        return Ok(());
    };

    let (stdin_sender, mut stdin_receiver) = mpsc::channel(1024);

    let mut context: ClientContext = ClientContext::default();

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

    let mut client_receiver = client
        .get_update_receiver()
        .expect("Could not get update receiver from the network client");

    let help_msg: String = format!(
        "\n┌────Help────────────────────────────
│\t {} - quit                        
│\t {} - show this message             
│\t {} - message the server you're ready
│\t {} - revoke readiness
│\t {} - send text message to the server
│\t {} - play alone
│
│\t {} <index>- choose a capture path from the list
│\t {} <index> <index>- choose move from the list. First number is pawn you want to move and second is position it should take
└───────────────────────────────────────
        ",
        QUIT_COMMAND, HELP_COMMAND, READY_COMMAND, UNREADY_COMMAND, SEND_COMMAND, SINGLEPLAYER_COMMAND , CAPTURE_COMMAND, MOVE_COMMAND
    );

    println!("───────Type /help for help───────");
    loop {
        tokio::select! {
        msg = client_receiver.recv() => {
            match msg {
                Some(ServerMessage::GameStart {
                    identity,
                }) => {
                    println!(
                        "Got new identity: {}",
                        identity
                    );
                    context.identity = Some(identity);
                }

                Some(ServerMessage::BroadcastBoardState{board}) => {
                    println!("New board state:\n{}", board.to_string(&context.identity.as_ref().unwrap().id));
                }

                Some(ServerMessage::BroadcastCurrentTurn{active_player}) => {
                    context.is_my_turn = active_player == context.identity.as_ref().expect("Server sent us new current turn without telling us who we are!").id;
                    if context.is_my_turn {
                        println!("It's our turn!");
                    } else {
                        println!("It's the turn of our enemy!");
                    }
                }

                Some(ServerMessage::AvailableCaptures{captures}) => {
                    println!("Available captures paths: {:?}", captures);
                    context.available_moves = None;
                    context.available_captures = Some(captures);
                }

                Some(ServerMessage::AvailableMoves{moves}) => {
                    println!("Available moves: {:?}", moves);
                    context.available_moves = Some(moves);
                    context.available_captures = None;
                },

                Some(ServerMessage::BroadCastTextMessage {sender, content}) => {
                    println!("[{}] - {}", sender, content);
                }

                Some(ServerMessage::GameEnd{result}) => {
                    println!("Game has ended with the result: {:?}", result);
                }

                Some(_) => {
                    error!("This shouldn't happen");
                } // ignore messages already handled by the network client

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

                    match CliCommands::try_from(cmd) {
                        Ok(cmd) => match cmd {
                            CliCommands::Quit => {
                                println!("Bye bye");
                                std::process::exit(0);
                            }
                            CliCommands::Help => {
                                println!("{}", help_msg);
                            }
                            CliCommands::Ready => {
                                client.signal_readiness().await;
                            }

                            CliCommands::Unready => {
                                client.revoke_readiness().await;
                            }

                            CliCommands::Single => {
                                println!("Tried connecting bot to the server! Make sure to be /ready");
                                tokio::spawn(run_bot());
                            }

                            CliCommands::Send => {
                                let mut args: String = words[1..].join(" ");
                                args = args.trim().to_string();
                                client.send_text_message(args).await;
                            }

                            CliCommands::Capture => {
                                if words.len() < 2 {
                                    error!("Too little arguments!");
                                    continue;
                                }

                                let arg = words[1];

                                match arg.parse::<usize>() {
                                    Ok(index) => {
                                        if let Some(captures) = context.available_captures.as_ref() && context.is_my_turn {
                                            if index >= captures.len() {
                                                error!("Cannot capture path that doesn't exist!");
                                                continue;
                                            }

                                            if !context.is_my_turn {
                                                error!("We cannot capture when it's not our turn!");
                                            }

                                            client.send_capture(captures[index].clone()).await;
                                        }
                                    },
                                    Err(_) => {
                                        error!("Invalid input! Use: /capture <index>, where index is the number from capture_path");
                                        continue;
                                    }
                                }
                            },
                            CliCommands::Move => {
                                if words.len() < 3 {
                                    error!("Too little arguments provided!");
                                    continue;
                                }

                                let arg = words[1];
                                let move_arg = words[2];


                                match arg.parse::<usize>() {
                                    Ok(index) => {
                                        if let Some(moves) = context.available_moves.as_ref() && context.is_my_turn {
                                            if index >= moves.len() {
                                                error!("Cannot make a move that doesn't exist!");
                                                continue;
                                            }

                                            if !context.is_my_turn {
                                                error!("We cannot make a move when it's not our turn!");
                                            }

                                            let move_path = moves[index].clone();

                                            match move_arg.parse::<usize>() {
                                                Ok(move_index) => {
                                                    if move_index >= move_path.available_steps.len() {
                                                        error!("Second index out of range!");
                                                        continue;
                                                    }

                                                    client.send_move(move_path.from, move_path.available_steps[move_index]).await;
                                                },
                                                Err(_) => {
                                                    error!("Invalid second index. Use: /move <index> <index>, where first index is the pawn you want to move and second index is position you want it to take");
                                                    continue;
                                                }
                                            }

                                        }
                                    },
                                    Err(_) => {
                                        error!("Invalid input! Use: /move <index> <index>, where first index is the pawn you want to move and second index is position you want it to take");
                                        continue;
                                    }
                                }

                            },
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
