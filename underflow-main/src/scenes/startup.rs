use comui::{
    component::Component,
    components::label::Label,
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
};
use macroquad::{color, miniquad::window::quit};

use crate::components::button::RoundedButton;

pub struct StartupScene {
    title: Label,
    start_btn: RoundedButton,
    settings_btn: RoundedButton,
    quit_btn: RoundedButton,

    next_scene: Option<NextScene>,
}

impl Default for StartupScene {
    fn default() -> Self {
        Self {
            title: Label::new("UNDERFLOW")
                .with_color(color::BLACK)
                .with_font_size(48.),
            start_btn: RoundedButton::default()
                .with_color(color::BLACK)
                .with_radius(0.2),
            settings_btn: RoundedButton::default()
                .with_color(color::BLACK)
                .with_radius(0.2),
            quit_btn: RoundedButton::default()
                .with_color(color::BLACK)
                .with_radius(0.2),

            next_scene: None,
        }
    }
}

impl StartupScene {
    const BUTTON_WIDTH: f32 = 0.3;
    const BUTTON_HEIGHT: f32 = 0.15;
    const BUTTON_GAP: f32 = 0.05;
}

impl Layout for StartupScene {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect((0.0, 0.4, 0.6, 0.3), &mut self.title)
            .at_rect(
                (0.0, 0.05, Self::BUTTON_WIDTH, Self::BUTTON_HEIGHT),
                &mut self.start_btn,
            )
            .at_rect(
                (
                    0.0,
                    0.05 - (Self::BUTTON_GAP + Self::BUTTON_HEIGHT),
                    Self::BUTTON_WIDTH,
                    Self::BUTTON_HEIGHT,
                ),
                &mut self.settings_btn,
            )
            .at_rect(
                (
                    0.0,
                    0.05 - (Self::BUTTON_GAP + Self::BUTTON_HEIGHT) * 2.0,
                    Self::BUTTON_WIDTH,
                    Self::BUTTON_HEIGHT,
                ),
                &mut self.quit_btn,
            )
            .build()
    }

    fn after_render(&mut self, _: &Transform, _: &mut comui::window::Window) {
        if self.quit_btn.inner.triggered {
            quit();
        }
        if self.start_btn.inner.triggered {
            // TODO: set next_scene
            self.start_btn.inner.triggered = false;
        }
    }
}

impl Scene for StartupScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        self.next_scene.take()
    }
}
