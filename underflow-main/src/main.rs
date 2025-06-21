underflow_l10n::tl_file!("common" tl crate::);

use comui::{
    component::Component,
    layout::{Layout, LayoutBuilder},
    scene::SceneManager,
    utils::Transform,
    window::Window,
};
use macroquad::{
    miniquad::EventHandler,
    prelude::info,
    window::{clear_background, next_frame, screen_height, screen_width},
};
use nalgebra::Matrix3;

use crate::{config::set_config, input::InputHandler, scenes::startup::StartupScene};

mod colors;
mod components;
mod config;
mod input;
mod scenes;
mod utils;

fn macroquad_config() -> macroquad::window::Conf {
    macroquad::window::Conf {
        high_dpi: true,
        sample_count: 4,
        window_title: "Underflow".to_string(),
        ..Default::default()
    }
}

struct Main {
    scene_manager: SceneManager,
}

impl Default for Main {
    fn default() -> Self {
        Self {
            scene_manager: SceneManager {
                scene_stack: vec![Box::new(StartupScene::default()) as Box<dyn comui::scene::Scene>],
            },
        }
    }
}

impl Layout for Main {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        let (w, h) = (screen_width(), screen_height());
        LayoutBuilder::new()
            .at_rect(
                (w / 2.0, h / 2.0, w, -h),
                &mut self.scene_manager as &mut dyn Component,
            )
            .build()
    }
}

#[macroquad::main(macroquad_config)]
async fn main() {
    set_config(Default::default());

    let mut handler = InputHandler::default();
    let mut main_view = Main::default();
    let mut window = Window::default();
    loop {
        handler.update();
        let touches = std::mem::take(&mut handler.touches);
        for touch in &touches {
            if let Err(e) = main_view.touch(touch) {
                info!("Error handling touch: {:?}", e);
            }
        }
        clear_background(macroquad::color::WHITE);
        main_view.render(&Matrix3::identity(), &mut window);
        window.update();
        next_frame().await
    }
}
