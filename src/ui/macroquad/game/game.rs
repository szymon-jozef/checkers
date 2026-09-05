use tokio::sync::mpsc::{self, Sender};

use log::{debug, error, info};
use macroquad::{
    color::GRAY,
    input::{KeyCode, get_keys_pressed},
    math::{Rect, Vec2, vec2},
    shapes::draw_rectangle,
    ui::{
        hash, root_ui,
        widgets::{Checkbox, Group},
    },
    window::{screen_height, screen_width},
};
use tokio::sync::mpsc::{Receiver, error::TryRecvError::Disconnected};

use crate::{
    logic::board::pawn::{CapturePath, MovePath},
    network::{
        client::Client, message::ServerMessage, network_identity::NetworkIdentity,
        server::ServerStage,
    },
    ui::{
        macroquad::{
            game::{board::Board, chat::Chat},
            menu_builder::MenuBuilder,
        },
        state::GameContext,
    },
};

pub struct GameClient {
    cmd_sender: Sender<GuiCommands>,
    update_receiver: Receiver<ServerMessage>,

    identity: Option<NetworkIdentity>,

    board: Board,
    is_my_turn: bool,

    game_state: ServerStage,

    game_area: Rect,
    board_area: Rect,

    lobby: Lobby,
    chat: Chat,

    game_result: Option<String>,
    pub should_close: bool,
}

#[derive(Default)]
struct Lobby {
    is_ready: bool,
    checkbox_pos: Vec2,
    checkbox_size: Vec2,
}

pub enum GuiCommands {
    Send(String),

    Ready,
    Unready,

    Capture(CapturePath),
    Move(MovePath),
}

impl GameClient {
    pub fn new(mut client: Client, _context: &GameContext) -> Option<Self> {
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
                            GuiCommands::Capture(capture_path) => {
                                client.send_capture(capture_path).await;
                            }

                            GuiCommands::Move(move_path) => {
                                client
                                    .send_move(move_path.from, move_path.available_steps[0])
                                    .await; // ugly
                            }
                        },

                        None => {
                            error!("Connection with cmd thread broken");
                            return;
                        }
                    }
                }
            });
        });

        let board = Board::default();

        let game_stage = ServerStage::default();
        let mut lobby = Lobby::default();
        lobby.checkbox_size = vec2(32.0, 16.0);

        let game_area = Rect {
            x: 0.0,
            y: 0.0,
            w: screen_width() * 0.8,
            h: screen_height(),
        };

        let board_area = Rect::default();

        let chat = Chat::default();

        Some(Self {
            cmd_sender,
            update_receiver,
            identity: None,

            board,
            lobby,

            is_my_turn: false,

            game_state: game_stage,

            game_area,
            board_area,

            chat,

            game_result: None,
            should_close: false,
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
        self.chat.draw();
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

    fn draw_game(&mut self) {
        self.board.draw();
    }

    fn draw_summary_screen(&mut self) {
        // this function doesn't only draw but i'm to lazy to change
        // this name everywhere
        let mut menu_builder = MenuBuilder::new(self.game_area.w, self.game_area.h * 0.1);
        let menu_pos = vec2(self.game_area.x, self.game_area.y);

        draw_rectangle(
            menu_pos.x,
            menu_pos.y,
            self.game_area.w,
            self.game_area.h,
            GRAY,
        );

        Group::new(hash!(), vec2(self.game_area.w, self.game_area.h))
            .position(menu_pos)
            .ui(&mut root_ui(), |ui| {
                menu_builder.label(ui, "Game ended!");

                if let Some(result) = &self.game_result {
                    menu_builder.label(ui, result);
                } else {
                    menu_builder.label(ui, "Could not get result");
                }

                if menu_builder.button(ui, "Main menu") {
                    // TODO! Should also close the connection
                    // and cleanup
                    self.should_close = true;
                }
            });
    }

    /* === UPDATING STUFF ==== */

    pub fn update(&mut self) {
        self.update_rects();
        self.update_network();

        match self.game_state {
            ServerStage::Lobby => self.update_lobby(),

            ServerStage::Game => {
                self.board.update();
            }

            ServerStage::End => {}
        }

        for key in get_keys_pressed() {
            if key == KeyCode::Enter {
                self.chat.send_message(self.cmd_sender.clone()); // not sure about cloning this
                // every message sent. Maybe
                // Chat should have it's own
                // clone?
            }
        }

        if let Some(capture_path) = self.board.get_choosen_capture() {
            let _ = self.cmd_sender.try_send(GuiCommands::Capture(capture_path));
        }

        if let Some(move_path) = self.board.get_choosen_move() {
            let _ = self.cmd_sender.try_send(GuiCommands::Move(move_path));
        }
    }

    fn update_network(&mut self) {
        match self.update_receiver.try_recv() {
            Ok(msg) => match msg {
                ServerMessage::GameStart { identity } => {
                    info!("We go identity!");
                    self.identity = Some(identity.clone());
                    self.board = Board::new(self.board_area, identity.id);
                    self.game_state = ServerStage::Game;
                }

                ServerMessage::AvailableCaptures { captures } => {
                    self.board.update_available_captures(captures);
                }

                ServerMessage::AvailableMoves { moves } => {
                    self.board.update_available_moves(moves);
                }

                ServerMessage::BroadcastBoardState { board } => {
                    self.board.update_board_view(board);
                    self.board.repopulate();
                }

                ServerMessage::BroadcastCurrentTurn { active_player } => {
                    if let Some(identity) = &self.identity
                        && identity.id == active_player
                    {
                        info!("It's out turn!");
                        self.is_my_turn = true;
                    } else {
                        self.is_my_turn = false;
                    }

                    self.board.set_my_turn(self.is_my_turn);
                }

                ServerMessage::BroadCastTextMessage { sender, content } => {
                    debug!("Got message from: {} with content: {}", sender, content);
                    self.chat.push_message(sender, content);
                }

                ServerMessage::GameEnd { result } => {
                    self.game_result = match result {
                        crate::logic::game_master::GameResult::Lost(uuid) => {
                            if self.identity.clone().unwrap().id == uuid {
                                Some("You lost!".to_string())
                            } else {
                                Some("You won!".to_string())
                            }
                        }
                        crate::logic::game_master::GameResult::Draw => Some("Draw!".to_string()),
                    };

                    self.game_state = ServerStage::End;
                }

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

        self.chat.update(&self.game_area);

        let w = self.game_area.w * 0.75;
        let h = self.game_area.h * 0.75;
        let side = w.min(h);

        self.board_area = Rect {
            x: self.game_area.w / 2.0 - (side / 2.0),
            y: self.game_area.h / 2.0 - (side / 2.0),
            w: side,
            h: side,
        };

        if self.board.update_board_rect(self.board_area) {
            self.board.update_dimensions();
        }
    }
}
