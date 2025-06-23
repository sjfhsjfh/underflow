use comui::{
    component::Component,
    components::label::{Align, Label},
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
    window::Window,
};

use crate::{colors, components::button::LabeledButton, scenes::setting::SettingScene};

pub struct PauseScene {
    resume_btn: LabeledButton,
    settings_btn: LabeledButton,
    quit_btn: LabeledButton,

    next_scene: Option<NextScene>,
}

impl Default for PauseScene {
    fn default() -> Self {
        let label_f = |l: Label| {
            l.with_align(Align::Center)
                .with_color(colors::WHITE)
                .with_font_size(54.)
                .with_line_height(54.)
                .with_texture_align((0.5, 0.6))
        };
        Self {
            resume_btn: LabeledButton::new_with_id("resume", label_f, |b| {
                b.with_color(colors::color_primary()).with_radius(0.5)
            }),
            settings_btn: LabeledButton::new_with_id("settings", label_f, |b| {
                b.with_color(colors::color_secondary()).with_radius(0.5)
            }),
            quit_btn: LabeledButton::new_with_id("exit-to-menu", label_f, |b| {
                b.with_color(colors::color_tertiary()).with_radius(0.5)
            }),
            next_scene: None,
        }
    }
}

impl Layout for PauseScene {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect((0.0, 0.2, 0.4, 0.15), &mut self.resume_btn)
            .at_rect((0.0, 0.0, 0.4, 0.15), &mut self.settings_btn)
            .at_rect((0.0, -0.2, 0.4, 0.15), &mut self.quit_btn)
            .build()
    }

    fn after_render(&mut self, _: &Transform, _: &mut Window) {
        if self.resume_btn.triggered() {
            self.next_scene = Some(NextScene::Pop);
        }
        if self.settings_btn.triggered() {
            self.next_scene = Some(NextScene::Push(Box::new(SettingScene::default())));
        }
        if self.quit_btn.triggered() {
            self.next_scene = Some(NextScene::PopN(2));
        }
    }
}

impl Scene for PauseScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        self.next_scene.take()
    }
}
