use tokio::sync::mpsc::{self, Sender};

use log::{error, info};
use macroquad::{
    color::GRAY,
    math::{Rect, Vec2, vec2},
    shapes::draw_rectangle,
    ui::{Ui, hash, root_ui, widgets::Checkbox},
    window::{screen_height, screen_width},
};
use tokio::sync::mpsc::{Receiver, error::TryRecvError::Disconnected};

use crate::{
    logic::board::board_view::BoardView,
    network::{
        client::Client, message::ServerMessage, network_identity::NetworkIdentity,
        server::ServerStage,
    },
    ui::state::GameContext,
};

pub struct GameClient {
    cmd_sender: Sender<GuiCommands>,
    update_receiver: Receiver<ServerMessage>,

    identity: Option<NetworkIdentity>,

    board: BoardView,
    player_name: String,
    enemy_name: String, // TODO! Server should give this info to us (it doesn't rn)

    game_state: ServerStage,

    game_area: Rect,
    game_padding: f32,

    lobby: Lobby,

    chat_area: Rect,
}

#[derive(Default)]
struct Lobby {
    is_ready: bool,
    checkbox_pos: Vec2,
    checkbox_size: Vec2,
}

enum GuiCommands {
    Send(String),

    Ready,
    Unready,

    Capture,
    Move,
}

impl GameClient {
    pub fn new(mut client: Client, context: &GameContext) -> Option<Self> {
        let Some(update_receiver) = client.get_update_receiver() else {
            return None;
        };

        let (cmd_sender, mut cmd_recv) = mpsc::channel::<GuiCommands>(10);

        std::thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap(); // i dont like this

            rt.block_on(async move {
                loop {
                    match cmd_recv.recv().await {
                        Some(cmd) => match cmd {
                            GuiCommands::Send(content) => {
                                client.send_text_message(content).await;
                            }

                            GuiCommands::Ready => {
                                client.signal_readiness().await;
                            }

                            GuiCommands::Unready => {
                                client.revoke_readiness().await;
                            }
                            GuiCommands::Capture => todo!(),
                            GuiCommands::Move => todo!(),
                        },

                        None => {
                            error!("Connection with cmd thread broken");
                            return;
                        }
                    }
                }
            });
        });

        let board = BoardView::default();

        let game_stage = ServerStage::default();
        let mut lobby = Lobby::default();
        lobby.checkbox_size = vec2(32.0, 16.0);

        let game_area = Rect {
            x: 0.0,
            y: 0.0,
            w: screen_width() * 0.8,
            h: screen_height(),
        };
        let game_padding = 25.0;

        let chat_area = Rect {
            x: game_area.w,
            y: 0.0,
            w: screen_width() * 0.2,
            h: screen_height(),
        };

        Some(Self {
            cmd_sender,
            update_receiver,
            identity: None,

            board,
            lobby,

            player_name: "Morbius".to_string(),
            enemy_name: "Milo".to_string(),

            game_state: game_stage,

            game_area,
            game_padding,

            chat_area,
        })
    }

    pub fn draw(&mut self) {
        match self.game_state {
            ServerStage::Lobby => {
                self.draw_lobby();
            }

            ServerStage::Game => {
                self.draw_game();
            }

            ServerStage::End => {
                self.draw_summary_screen();
            }
        }
        //self.draw_chat(); // TODO! uncomment this
    }

    fn draw_lobby(&mut self) {
        let lobby = &self.lobby;

        let background_x = lobby.checkbox_size.x * 10.0;
        let background_y = lobby.checkbox_size.y * 10.0;

        draw_rectangle(
            lobby.checkbox_pos.x - background_x * 0.5,
            lobby.checkbox_pos.y - background_y * 0.5,
            background_x,
            background_y,
            GRAY,
        );

        let before_click: bool = self.lobby.is_ready;
        Checkbox::new(hash!())
            .pos(self.lobby.checkbox_pos)
            .label("Ready")
            .size(self.lobby.checkbox_size)
            .ui(&mut root_ui(), &mut self.lobby.is_ready);

        if before_click != self.lobby.is_ready {
            if self.lobby.is_ready {
                let _ = self.cmd_sender.try_send(GuiCommands::Ready); // TODO! Maybe handle failure
            // or something
            } else {
                let _ = self.cmd_sender.try_send(GuiCommands::Unready);
            }
        }
    }

    fn draw_game(&self) {
        todo!();
    }

    fn draw_summary_screen(&self) {
        todo!();
    }

    fn draw_chat(&self) {
        todo!();
    }

    /* === UPDATING STUFF ==== */

    pub fn update(&mut self) {
        self.update_rects();
        self.update_network();

        match self.game_state {
            ServerStage::Lobby => self.update_lobby(),
            ServerStage::Game => todo!(),
            ServerStage::End => todo!(),
        }
    }

    fn update_network(&mut self) {
        match self.update_receiver.try_recv() {
            Ok(msg) => match msg {
                ServerMessage::GameStart { identity } => {
                    info!("We go identity!");
                    self.identity = Some(identity);
                }

                ServerMessage::AvailableCaptures { captures } => todo!(),
                ServerMessage::AvailableMoves { moves } => todo!(),
                ServerMessage::BroadcastBoardState { board } => todo!(),
                ServerMessage::BroadcastCurrentTurn { active_player } => todo!(),
                ServerMessage::BroadCastTextMessage { sender, content } => todo!(),
                ServerMessage::GameEnd { result } => todo!(),

                _ => {} // ignore things that network client handled by itself
            },

            Err(Disconnected) => {
                error!("Connection broken!!!!");
            }

            Err(_) => {} // No messages - nothing wrong (why is this even an error)
        }
    }

    fn update_lobby(&mut self) {
        self.lobby.checkbox_pos = vec2(
            self.game_area.w / 2.0 - self.lobby.checkbox_size.x / 2.0,
            self.game_area.h / 2.0,
        );
    }

    fn update_rects(&mut self) {
        self.game_area = Rect {
            x: 0.0,
            y: 0.0,
            w: screen_width() * 0.8,
            h: screen_height(),
        };

        self.chat_area = Rect {
            x: self.game_area.w,
            y: 0.0,
            w: screen_width() * 0.2,
            h: screen_height(),
        };
    }
}
