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
    components::button::LabeledButton,
    scenes::{preflight::PreflightScene, setting::SettingScene},
};

pub struct StartupScene {
    title: Label,
    start_btn: LabeledButton,
    settings_btn: LabeledButton,
    quit_btn: LabeledButton,

    next_scene: Option<NextScene>,
}

impl Default for StartupScene {
    fn default() -> Self {
        Self {
            title: Label::new("UNDERFLOW")
                .with_align(Align::Center)
                .with_color(colors::BLACK)
                .with_font_size(Self::TITLE_SIZE),
            start_btn: LabeledButton::new_with_id("start-game", Self::button_text_label, |b| {
                b.with_color(colors::color_primary()).with_radius(0.5)
            }),

            settings_btn: LabeledButton::new_with_id("settings", Self::button_text_label, |b| {
                b.with_color(colors::color_secondary()).with_radius(0.5)
            }),

            quit_btn: LabeledButton::new_with_id("quit", Self::button_text_label, |b| {
                b.with_color(colors::color_tertiary()).with_radius(0.5)
            }),

            next_scene: None,
        }
    }
}

impl StartupScene {
    const BUTTON_WIDTH: f32 = 0.4;
    const BUTTON_HEIGHT: f32 = 0.125;
    const BUTTON_GAP: f32 = 0.05;

    const TITLE_SIZE: f32 = 128.0;
    const BUTTON_LABEL_SIZE: f32 = 72.0;

    fn button_text_label(label: Label) -> Label {
        label
            .with_align(Align::Center)
            .with_color(colors::WHITE)
            .with_texture_align((0.5, 0.6))
            .with_line_height(Self::BUTTON_LABEL_SIZE)
            .with_font_size(Self::BUTTON_LABEL_SIZE)
    }
}

impl Layout for StartupScene {
    fn before_render(&mut self, tr: &Transform, _: &mut comui::window::Window) {
        self.title.area_width = Some(tr.transform_vector(&Vector2::new(1.0, 0.0)).norm());
    }

    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect((0.0, 0.28, 0.6, 0.3), &mut self.title as &mut dyn Component)
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
        if self.quit_btn.triggered() {
            quit();
        }
        if self.start_btn.triggered() {
            self.next_scene = Some(NextScene::Push(
                Box::new(PreflightScene::default()) as Box<dyn Scene>
            ));
        }
        if self.settings_btn.triggered() {
            self.next_scene = Some(NextScene::Push(
                Box::new(SettingScene::default()) as Box<dyn Scene>
            ));
        }
    }
}

impl Scene for StartupScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        self.next_scene.take()
    }
}
