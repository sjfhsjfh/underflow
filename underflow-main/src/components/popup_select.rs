use comui::{component::Component, components::DataComponent, layout::Layout, utils::Transform};

pub struct PopUpSelect<D> {
    pub selected: usize,
    pub options: Vec<D>,
}

impl<D: PartialEq> DataComponent<D> for PopUpSelect<D> {
    fn get_data(&self) -> &D {
        &self.options[self.selected]
    }

    fn set_data(&mut self, data: D) {
        self.selected = self.options.iter().position(|x| x == &data).unwrap();
    }
}

impl Layout for PopUpSelect<String> {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        vec![]
    }
}
