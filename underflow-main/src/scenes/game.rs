use macroquad::{
    math::{Rect, vec2},
    prelude::debug,
    ui::hash,
    window::screen_width,
};
use underflow_core::Board;

use crate::ui::{Color, Ui, button::DRectButton};

use super::{NextScene, Scene};

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

#[derive(Clone)]
pub struct GameInitConfig {
    pub player_count: u8,
    pub size: u8,
}

pub struct GameData {
    pub board: Board,
    pub config: GameInitConfig,
    pub pause_btn: DRectButton,
}

impl GameData {
    #[inline]
    pub fn next_player(&self, current: u8) -> u8 {
        (current + 1) % self.config.player_count
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum GameState {
    #[default]
    Filling,
    Operating,
}

impl GameState {
    pub fn is_filling(&self) -> bool {
        matches!(self, Self::Filling)
    }
}

pub(crate) struct GameScene {
    data: GameData,

    state: GameState,
    current_player_filled: bool,
    current_player: u8,
}

impl GameScene {
    pub fn new(config: GameInitConfig) -> Self {
        // Initialize fields here
        Self {
            data: GameData {
                board: Board::init(config.player_count, config.size),
                config,

                pause_btn: DRectButton::new(),
            },
            state: GameState::default(),
            current_player_filled: false,
            current_player: 0,
        }
    }

    #[inline]
    pub fn next_player(&mut self) {
        self.current_player = self.data.next_player(self.current_player);
        self.current_player_filled = false;
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
                let cell = self.data.board.get(x, y);
                let r = self.cell_rect(x, y);
                let color = match cell {
                    underflow_core::CellState::Empty => Color::new(0.9, 0.9, 0.9, 1.0),
                    underflow_core::CellState::Occupied(player) => COLOR_MAP[player as usize],
                    underflow_core::CellState::Anchored(player) => {
                        COLOR_MAP[player as usize].darken(0.3)
                    }
                    underflow_core::CellState::Neutral => Color::new(0.5, 0.5, 0.5, 1.0),
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
        match self.state {
            GameState::Filling => {
                if self.current_player_filled {
                    return Ok(false);
                }
                for x in 0..self.data.config.size {
                    for y in 0..self.data.config.size {
                        let r = self.cell_rect(x, y);
                        if r.contains(touch.position) {
                            if self.data.board.get(x, y) != underflow_core::CellState::Empty {
                                return Ok(false);
                            }
                            self.current_player_filled = true;
                            self.data.board.set(
                                x,
                                y,
                                underflow_core::CellState::Occupied(self.current_player),
                            );
                            // Remove this
                            self.next_player();
                            return Ok(true);
                        }
                    }
                }
            }
            GameState::Operating => {
                // Implement touch handling logic here
            }
        }
        Ok(false)
    }
}
