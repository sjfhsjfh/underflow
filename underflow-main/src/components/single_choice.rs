use comui::{
    component::Component,
    components::{DataComponent, button::QuadButton},
    layout::{Layout, LayoutBuilder},
    utils::Transform,
};

pub struct SingleChoice {
    choices: Vec<String>,
    selected: usize,
    btn: QuadButton,
}

impl SingleChoice {
    pub fn new(choices: Vec<String>, selected: usize) -> Self {
        Self {
            choices,
            selected,
            btn: QuadButton::default(),
        }
    }

    pub fn updated(&mut self) -> bool {
        if self.btn.triggered {
            self.btn.triggered = false;
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
