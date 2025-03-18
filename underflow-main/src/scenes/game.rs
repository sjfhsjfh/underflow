use macroquad::{math::Rect, prelude::debug};
use underflow_core::{
    CellState,
    protocol::{FlowCommand, GamePhase},
    server::{FlowServer, FlowServerConfig},
};

use super::{NextScene, Scene};
use crate::ui::{Color, Ui};

static COLOR_MAP: [Color; 6] = [
    Color::new(1.0, 0.0, 0.0, 1.0),
    Color::new(0.0, 1.0, 0.0, 1.0),
    Color::new(0.0, 0.0, 1.0, 1.0),
    Color::new(1.0, 1.0, 0.0, 1.0),
    Color::new(1.0, 0.0, 1.0, 1.0),
    Color::new(0.0, 1.0, 1.0, 1.0),
];

const BOARD_UI_SIZE: f32 = 0.6;
const GUTTER_RATIO: f32 = 0.15;

pub struct GameData {
    pub server: FlowServer,
    pub config: FlowServerConfig,
}

pub(crate) struct GameScene {
    data: GameData,
    current_player_filled: bool,
}

impl GameScene {
    fn current_player(&self) -> u8 {
        self.data.server.current_player
    }

    fn get_cell(&self, x: u8, y: u8) -> CellState {
        self.data.server.board.get(x, y)
    }

    pub fn new(config: FlowServerConfig) -> Self {
        // Initialize fields here
        Self {
            data: GameData {
                server: FlowServer::new(config),
                config,
            },
            current_player_filled: false,
        }
    }

    #[inline]
    pub fn cell_size(&self) -> f32 {
        BOARD_UI_SIZE / (self.data.config.size as f32 * (1. + GUTTER_RATIO) - GUTTER_RATIO)
    }

    #[inline]
    pub fn cell_rect(&self, x: u8, y: u8) -> Rect {
        let cell_size = self.cell_size();
        Rect::new(
            -0.5 * BOARD_UI_SIZE + x as f32 * cell_size * (1. + GUTTER_RATIO),
            0.5 * BOARD_UI_SIZE - (y as f32 + 1.) * cell_size * (1. + GUTTER_RATIO),
            cell_size,
            cell_size,
        )
    }

    pub fn draw_board(&self, ui: &mut Ui) {
        for x in 0..self.data.config.size {
            for y in 0..self.data.config.size {
                let cell = self.data.server.board.get(x, y);
                let r = self.cell_rect(x, y);
                let color = match cell {
                    CellState::Empty => Color::new(0.9, 0.9, 0.9, 1.0),
                    CellState::Occupied(player) => COLOR_MAP[player as usize],
                    CellState::Anchored(player) => COLOR_MAP[player as usize].darken(0.3),
                    CellState::Neutral => Color::new(0.5, 0.5, 0.5, 1.0),
                };
                ui.fill_rect(r, color);
            }
        }
    }
}

impl Scene for GameScene {
    fn next_scene(&self) -> NextScene {
        // Implement scene transition logic here
        NextScene::None
    }

    fn render(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        self.draw_board(ui);
        Ok(())
    }

    fn touch(&mut self, touch: &macroquad::prelude::Touch) -> anyhow::Result<bool> {
        match self.data.server.phase {
            GamePhase::Filling => {
                if self.current_player_filled {
                    return Ok(false);
                }
                for x in 0..self.data.config.size {
                    for y in 0..self.data.config.size {
                        let r = self.cell_rect(x, y);
                        if r.contains(touch.position) {
                            if self.get_cell(x, y) != CellState::Empty {
                                return Ok(false);
                            }
                            // TODO: add this and make it false after the transition animation
                            // self.current_player_filled = true;
                            let res = self.data.server.handle(FlowCommand::SetOccupied {
                                player: self.current_player(),
                                x,
                                y,
                            });
                            match res {
                                Err(e) => debug!("{}", e),
                                _ => {}
                            }
                            return Ok(true);
                        }
                    }
                }
            }
            GamePhase::Flowing => {
                // Implement touch handling logic here
            }
        }
        Ok(false)
    }
}
