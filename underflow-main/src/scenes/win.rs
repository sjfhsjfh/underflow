use comui::{
    component::Component,
    components::label::{Align, Label},
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
};
use macroquad::color::Color;

use crate::{colors, components::button::LabeledButton, tl};

pub struct WinScene {
    winning_label: Label,
    return_btn: LabeledButton,

    next_scene: Option<NextScene>,
}

impl WinScene {
    pub fn new(color: Color) -> Self {
        Self {
            winning_label: Label::new(tl!("you-win"))
                .with_align(Align::Center)
                .with_color(color)
                .with_font_size(96.),
            return_btn: LabeledButton::new_with_id(
                "exit-to-menu",
                |l| {
                    l.with_color(colors::WHITE)
                        .with_font_size(48.)
                        .with_line_height(48.)
                        .with_align(Align::Center)
                        .with_texture_align((0.5, 0.6))
                },
                |b| b.with_color(colors::color_primary()),
            ),

            next_scene: None,
        }
    }
}

impl Layout for WinScene {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect((0.0, 0.3, 0.9, 0.3), &mut self.winning_label)
            .at_rect((0.0, -0.1, 0.3, 0.1), &mut self.return_btn)
            .build()
    }

    fn after_render(&mut self, _: &Transform, _: &mut comui::window::Window) {
        if self.return_btn.triggered() {
            self.next_scene = Some(NextScene::Pop);
        }
    }
}

impl Scene for WinScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        self.next_scene.take()
    }
}
