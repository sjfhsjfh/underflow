use comui::{
    component::Component,
    components::label::{Align, Label},
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
};
use macroquad::miniquad::window::quit;
use nalgebra::Vector2;

use crate::{
    colors,
    components::button::RoundedButton,
    scenes::{preflight::PreflightScene, setting::SettingScene},
};

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
                .with_align(Align::Center)
                .with_color(colors::BLACK)
                .with_font_size(72.),
            start_btn: RoundedButton::default()
                .with_color(colors::BLACK)
                .with_radius(0.2),
            settings_btn: RoundedButton::default()
                .with_color(colors::BLACK)
                .with_radius(0.2),
            quit_btn: RoundedButton::default()
                .with_color(colors::BLACK)
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
    fn before_render(&mut self, tr: &Transform, _: &mut comui::window::Window) {
        self.title.area_width = Some(tr.transform_vector(&Vector2::new(1.0, 0.0)).norm());
    }

    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect((0.0, 0.25, 0.6, 0.3), &mut self.title as &mut dyn Component)
            .at_rect(
                (0.0, 0.05, Self::BUTTON_WIDTH, Self::BUTTON_HEIGHT),
                &mut self.start_btn as &mut dyn Component,
            )
            .at_rect(
                (
                    0.0,
                    0.05 - (Self::BUTTON_GAP + Self::BUTTON_HEIGHT),
                    Self::BUTTON_WIDTH,
                    Self::BUTTON_HEIGHT,
                ),
                &mut self.settings_btn as &mut dyn Component,
            )
            .at_rect(
                (
                    0.0,
                    0.05 - (Self::BUTTON_GAP + Self::BUTTON_HEIGHT) * 2.0,
                    Self::BUTTON_WIDTH,
                    Self::BUTTON_HEIGHT,
                ),
                &mut self.quit_btn as &mut dyn Component,
            )
            .build()
    }

    fn after_render(&mut self, _: &Transform, _: &mut comui::window::Window) {
        if self.quit_btn.inner.triggered {
            quit();
        }
        if self.start_btn.inner.triggered {
            self.next_scene = Some(NextScene::Push(
                Box::new(PreflightScene::default()) as Box<dyn Scene>
            ));
            self.start_btn.inner.triggered = false;
        }
        if self.settings_btn.inner.triggered {
            self.next_scene = Some(NextScene::Push(
                Box::new(SettingScene::default()) as Box<dyn Scene>
            ));
            self.settings_btn.inner.triggered = false;
        }
    }
}

impl Scene for StartupScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        self.next_scene.take()
    }
}
