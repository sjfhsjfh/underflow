use macroquad::{color::Color, math::vec2, shapes::draw_circle, ui::hash, window::screen_width};
use underflow_core::Board;

use super::{NextScene, Scene};

pub struct GameInitConfig {
    pub size: u8,
}

pub struct GameData {
    pub board: Board,
    pub config: GameInitConfig,
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
            },
        }
    }
}

impl Scene for GameScene {
    fn next_scene(&self) -> NextScene {
        // Implement scene transition logic here
        NextScene::None
    }

    fn render(&mut self, ui: &mut macroquad::ui::Ui) -> anyhow::Result<()> {
        // Implement rendering logic here
        // ui.label(macroquad::math::vec2(30., 30.), "Game Scene");
        ui.window(
            hash!(),
            vec2(0., 0.),
            vec2(screen_width(), screen_width()),
            |ui| {
                draw_circle(0., 0., 20., Color::from_hex(0x000000));
            },
        );
        Ok(())
    }
}
