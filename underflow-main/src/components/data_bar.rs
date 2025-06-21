use std::marker::PhantomData;

use comui::{
    component::Component,
    components::{
        DataComponent,
        label::{Align, Label},
    },
    layout::{Layout, LayoutBuilder},
    shading::IntoShading,
    utils::Transform,
};
use lyon::{
    geom::euclid::Box2D,
    path::{Path, Winding, builder::BorderRadii},
};
use macroquad::color;
use nalgebra::{Point2, Vector2};

pub struct DataBar<D, C: DataComponent<D> + Component> {
    pub name: Label,
    pub data: C,
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
            _marker: PhantomData,
        }
    }
}

impl<D, C: DataComponent<D> + Component> Layout for DataBar<D, C> {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect((-0.4, 0.0, 0.5, 0.8), &mut self.name as &mut dyn Component)
            .at_rect((0.3, 0.0, 0.3, 0.8), &mut self.data as &mut dyn Component)
            .build()
    }

    fn before_render(&mut self, tr: &Transform, target: &mut comui::window::Window) {
        let bottom_left = tr.transform_point(&(Point2::new(-0.5, 0.5)));
        let top_right = tr.transform_point(&(Point2::new(0.5, -0.5)));
        let height = tr.transform_vector(&Vector2::new(0.0, 1.0)).norm();
        let path = {
            let mut builder = Path::builder();
            builder.add_rounded_rectangle(
                &Box2D::new(
                    (bottom_left.x, bottom_left.y).into(),
                    (top_right.x, top_right.y).into(),
                ),
                &BorderRadii::new(0.5 * height),
                Winding::Positive,
            );
            builder.build()
        };
        target.stroke_path(&path, 1.0, 2.0, color::BLACK.into_shading());
    }
}
