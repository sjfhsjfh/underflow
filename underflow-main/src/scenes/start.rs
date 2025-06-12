use macroquad::{camera::set_camera, math::Rect, prelude::debug};
use underflow_core::server::FlowServerConfig;

use crate::ui::{Color, Ui, button::DRectButton};

use super::{NextScene, Scene};

pub(crate) struct StartScene {
    enter_game: bool,

    test_btn: DRectButton,
    game_config: FlowServerConfig,
}

impl StartScene {
    pub fn new() -> Self {
        Self {
            enter_game: false,
            test_btn: DRectButton::new(),
            game_config: FlowServerConfig {
                size: 7,
                player_count: 3,
            },
        }
    }
}

impl Scene for StartScene {
    fn next_scene(&self) -> NextScene {
        if self.enter_game {
            debug!("Entering game scene");
            NextScene::Replace(Box::new(super::GameScene::new(self.game_config)))
        } else {
            NextScene::None
        }
    }

    fn render(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        let cam = ui.camera();
        let top = -1. / cam.zoom.y;
        set_camera(&cam);

        let test_r = Rect::new(-0.5, -0.5 * top, 1., top);
        self.test_btn.build(ui, 0., test_r, |ui, path| {
            ui.fill_path(&path, Color::new(0.3, 0.3, 0.3, 1.));
        });

        Ok(())
    }

    fn touch(&mut self, touch: &macroquad::prelude::Touch) -> anyhow::Result<bool> {
        if self.test_btn.touch(touch, 0.) {
            debug!("Test button touched");
            self.enter_game = true;
            return Ok(true);
        }
        Ok(false)
    }
}
