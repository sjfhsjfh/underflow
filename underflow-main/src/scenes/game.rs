use macroquad::{
    color::Color,
    math::{Rect, vec2},
    prelude::debug,
    ui::hash,
    window::screen_width,
};
use underflow_core::Board;

use crate::ui::{Ui, button::DRectButton};

use super::{NextScene, Scene};

pub struct GameInitConfig {
    pub size: u8,
}

pub struct GameData {
    pub board: Board,
    pub config: GameInitConfig,
    pub pause_btn: DRectButton,
}

pub(crate) struct GameScene {
    data: GameData,
}

impl GameScene {
    pub fn new(config: GameInitConfig) -> Self {
        // Initialize fields here
        Self {
            data: GameData {
                board: Board::new(config.size),
                config,

                pause_btn: DRectButton::new(),
            },
        }
    }

    pub fn draw_board() {}
}

impl Scene for GameScene {
    fn next_scene(&self) -> NextScene {
        // Implement scene transition logic here
        NextScene::None
    }

    fn render(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        Ok(())
    }
}
