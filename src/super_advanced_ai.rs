use log::{error, info};
use rand::seq::IndexedRandom;
use uuid::Uuid;

use crate::{
    logic::game_master::GameResult,
    network::client::Client,
    settings::{client_settings::ClientSettings, general_settings::DEFAULT_URL},
};

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub enum BotDificulty {
    #[default]
    Easy,
    Normal,
    Hard,
}

pub struct Bot {
    dificulty: BotDificulty,
}

impl Bot {
    pub async fn new(dificulty: BotDificulty) -> Self {
        if dificulty == BotDificulty::Normal || dificulty == BotDificulty::Hard {
            todo!("Only easy is implemented");
        }

        Self { dificulty }
    }

    pub async fn game_loop(&self) -> Option<Uuid> {
        let mut client = Client::new(Some(ClientSettings {
            name: "Bot".to_string(),
            server_url: DEFAULT_URL,
        }))
        .await
        .expect("Couldn't connect to the server. Is it online?");

        client.signal_readiness().await; // bot is always ready

        let mut receiver = client
            .get_update_receiver()
            .expect("Could not receive the receiver");

        let dificulty = self.dificulty;

        loop {
            match receiver.recv().await {
                Some(message) => match message {
                    crate::network::message::ServerMessage::GameStart { identity } => {
                        info!("Bot got identity: {}", identity);
                    }
                    crate::network::message::ServerMessage::AvailableCaptures { captures } => {
                        let capture = captures
                            .choose(&mut rand::rng())
                            .expect("Could not select random capture");
                        client.send_capture(capture.clone()).await;
                    }
                    crate::network::message::ServerMessage::AvailableMoves { moves } => {
                        let rand_pawn = moves
                            .choose(&mut rand::rng())
                            .expect("Couldn't select random move");
                        let rand_move = rand_pawn
                            .available_steps
                            .choose(&mut rand::rng())
                            .expect("Could not get random move");

                        client.send_move(rand_pawn.from, rand_move.clone()).await;
                    }
                    crate::network::message::ServerMessage::BroadcastBoardState { board: _ } => {
                        info!("Got server state!");

                        match dificulty.clone() {
                            BotDificulty::Easy => {} // we do nothing with it because easy bot
                            // just does random stuff. In future
                            // board_view can be used to calculate the
                            // best possible move atm
                            BotDificulty::Normal => todo!(),
                            BotDificulty::Hard => todo!(),
                        }
                    }
                    crate::network::message::ServerMessage::BroadcastCurrentTurn {
                        active_player: _,
                    } => {} // we do nothing with this - bot doesn't need to know if it's the
                    // active player, as it's only responding to messages
                    crate::network::message::ServerMessage::BroadCastTextMessage {
                        sender,
                        content,
                    } => {
                        info!("Bot got message from [{}] - {}", sender, content);
                    }

                    crate::network::message::ServerMessage::GameEnd { result } => {
                        info!("Game has ended!");
                        match result {
                            GameResult::Draw => {
                                info!("Draw!");
                                return None;
                            }
                            GameResult::Lost(loser) => {
                                info!("ID: {} lost!", loser);
                                return Some(loser);
                            }
                        }
                    }

                    _ => {}
                },

                None => {
                    error!("Connection broken! Super advanced ai logging out!");
                    return None;
                }
            }
        }
    }
}
