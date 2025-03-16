use macroquad::{math::vec2, prelude::debug};

use crate::scenes::game::GameInitConfig;

use super::{NextScene, Scene};

pub(crate) struct StartScene {
    enter_game: bool,
}

impl StartScene {
    pub fn new() -> Self {
        Self { enter_game: false }
    }
}

impl Scene for StartScene {
    fn next_scene(&self) -> NextScene {
        if self.enter_game {
            debug!("Entering game scene");
            NextScene::Replace(Box::new(super::GameScene::new(GameInitConfig { size: 5 })))
        } else {
            NextScene::None
        }
    }

    fn render(&mut self, ui: &mut macroquad::ui::Ui) -> anyhow::Result<()> {
        ui.label(vec2(30., 30.), "Hello, world!");
        if ui.button(vec2(30., 60.), "Enter") {
            self.enter_game = true;
        };
        Ok(())
    }
}
