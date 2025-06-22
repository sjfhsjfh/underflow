use std::marker::PhantomData;

use comui::{
    component::Component,
    components::{
        DataComponent,
        label::{Align, Label},
    },
    layout::{Layout, LayoutBuilder},
    utils::Transform,
};
use macroquad::color;

use crate::{colors, components::rounded_rect::RoundedRect};

pub struct DataBar<D, C: DataComponent<D> + Component> {
    pub name: Label,
    pub data: C,

    container: RoundedRect,
    data_container: RoundedRect,

    _marker: PhantomData<D>,
}

impl<D, C: DataComponent<D> + Component> DataBar<D, C> {
    pub fn new(name: String, data: C) -> Self {
        Self {
            name: Label::new(&name)
                .with_align(Align::Left)
                .with_texture_align((0.0, 0.5))
                .with_font_size(48.)
                .with_line_height(30.)
                .with_color(color::BLACK),
            data,
            container: RoundedRect::builder()
                .with_radius_rel(0.0, 0.5)
                .with_fill_color(colors::color_secondary_container())
                .build(),
            data_container: RoundedRect::builder()
                .with_radius_rel(0.0, 0.5)
                .with_stroke(colors::BLACK, 2.0)
                .build(),
            _marker: PhantomData,
        }
    }
}

impl<D, C: DataComponent<D> + Component> Layout for DataBar<D, C> {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        let data_rect = (0.32, 0.0, 0.3, 0.7);
        LayoutBuilder::new()
            .at_rect((0.0, 0.0, 1.0, 1.0), &mut self.container)
            .at_rect(data_rect, &mut self.data_container)
            .at_rect((-0.4, 0.0, 0.5, 0.8), &mut self.name)
            .at_rect(data_rect, &mut self.data)
            .build()
    }
}
