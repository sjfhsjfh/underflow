use comui::{
    component::Component,
    components::{DataComponent, label::Align},
    layout::{Layout, LayoutBuilder},
    utils::Transform,
};

use crate::colors;

use super::button::LabeledButton;

pub struct SingleChoice {
    choices: Vec<String>,
    selected: usize,
    btn: LabeledButton,
}

impl SingleChoice {
    const BUTTON_LABEL_SIZE: f32 = 60.0;

    pub fn new(choices: Vec<String>, selected: usize) -> Self {
        Self {
            choices,
            selected,
            btn: LabeledButton::new_with_id("switch", |l| {
                l
                .with_align(Align::Center)
                .with_color(colors::WHITE)
                .with_texture_align((0.5, 0.6))
                .with_line_height(Self::BUTTON_LABEL_SIZE)
                .with_font_size(Self::BUTTON_LABEL_SIZE)
            }, |b| {
                b.with_color(colors::color_secondary()).with_radius(0.5f32)
            }),
        }
    }

    pub fn updated(&mut self) -> bool {
        if self.btn.triggered() {
            //self.btn.inner.inner.triggered = false;
            self.selected = (self.selected + 1) % self.choices.len();
            true
        } else {
            false
        }
    }
}

impl DataComponent<String> for SingleChoice {
    fn get_data(&self) -> &String {
        &self.choices[self.selected]
    }

    fn set_data(&mut self, data: String) {
        self.selected = self.choices.iter().position(|x| *x == data).unwrap();
    }
}

impl Layout for SingleChoice {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect((0.0, 0.0, 1.0, 1.0), &mut self.btn as &mut dyn Component)
            .build()
    }
}
