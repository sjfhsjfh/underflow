use comui::{component::Component, shading::IntoShading, utils::Transform, window::Window};
use lyon::{
    math::Box2D,
    path::{Path, Winding, builder::BorderRadii},
};
use macroquad::prelude::Touch;
use nalgebra::Vector2;

use crate::utils::UTransform;

pub struct RoundedRect {
    pub radius_rel_w: f32,
    pub radius_rel_h: f32,
    pub radius_abs: f32,
    pub fill_color: Option<macroquad::color::Color>,
    /// color, abs_px_width
    pub stroke: Option<(macroquad::color::Color, f32)>,
}

#[must_use = "Call `build` to finalize the rounded rectangle configuration"]
#[derive(Default)]
pub struct RoundedRectBuilder {
    radius_rel_w: f32,
    radius_rel_h: f32,
    radius_abs: f32,
    fill_color: Option<macroquad::color::Color>,
    stroke: Option<(macroquad::color::Color, f32)>,
}

impl RoundedRectBuilder {
    pub fn with_radius_rel(mut self, rel_w: f32, rel_h: f32) -> Self {
        self.radius_rel_w = rel_w;
        self.radius_rel_h = rel_h;
        self
    }

    pub fn with_radius_abs(mut self, abs: f32) -> Self {
        self.radius_abs = abs;
        self
    }

    pub fn with_fill_color(mut self, color: macroquad::color::Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn with_stroke(mut self, color: macroquad::color::Color, abs_px_width: f32) -> Self {
        self.stroke = Some((color, abs_px_width));
        self
    }

    pub fn build(self) -> RoundedRect {
        RoundedRect {
            radius_rel_w: self.radius_rel_w,
            radius_rel_h: self.radius_rel_h,
            radius_abs: self.radius_abs,
            fill_color: self.fill_color,
            stroke: self.stroke,
        }
    }
}

impl RoundedRect {
    pub fn builder() -> RoundedRectBuilder {
        Default::default()
    }
}

impl Component for RoundedRect {
    fn touch(&mut self, _: &Touch) -> anyhow::Result<bool> {
        Ok(false)
    }

    fn render(&mut self, tr: &Transform, target: &mut Window) {
        if self.fill_color.is_none() && self.stroke.is_none() {
            return;
        }
        let abs_w = tr.transform_vector(&Vector2::new(1.0, 0.0)).norm();
        let abs_h = tr.transform_vector(&Vector2::new(0.0, 1.0)).norm();
        let tr_rest = UTransform::new(
            tr * Transform::new_nonuniform_scaling(&Vector2::new(1.0 / abs_w, 1.0 / abs_h)),
        );
        let radius = abs_w * self.radius_rel_w + abs_h * self.radius_rel_h + self.radius_abs;
        let mut builder = Path::builder();
        let corner = (-0.5 * abs_w, -0.5 * abs_h).into();
        builder.add_rounded_rectangle(
            &Box2D::new(corner, -corner),
            &BorderRadii::new(radius),
            Winding::Positive,
        );
        let path = builder.build().transformed(&tr_rest);
        if let Some(fill_color) = self.fill_color {
            target.fill_path(&path, fill_color.into_shading(), 1.0);
        }
        if let Some((stroke_color, thickness)) = self.stroke {
            target.stroke_path(&path, stroke_color.into_shading(), 1.0, thickness);
        }
    }
}
