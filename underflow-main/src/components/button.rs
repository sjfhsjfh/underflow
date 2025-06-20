use comui::{
    components::button::QuadButton,
    layout::{Layout, LayoutBuilder},
    shading::IntoShading,
    utils::Transform,
    window::Window,
};
use lyon::{
    math::Box2D,
    path::{Path, Winding, builder::BorderRadii},
};
use macroquad::color::{self, Color};
use nalgebra::{Point2, Vector2};

/// Asserts the target region is a rectangle
pub struct RoundedButton {
    pub inner: QuadButton,
    pub color: Color,
    /// Relative to height
    pub radius: f32,
}

impl RoundedButton {
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl Default for RoundedButton {
    fn default() -> Self {
        Self {
            radius: 0.2,
            color: color::BLACK,
            inner: QuadButton::default(),
        }
    }
}

impl Layout for RoundedButton {
    fn components(&mut self) -> Vec<(Transform, &mut dyn comui::component::Component)> {
        LayoutBuilder::new()
            .at_rect((0.0, 0.0, 1.0 - self.radius, 1.0), &mut self.inner)
            .build()
    }

    fn before_render(&mut self, tr: &Transform, target: &mut Window) {
        let size = 1.0
            - 0.04 * {
                let t = if self.inner.pressed {
                    self.inner.press_start_at.elapsed().as_secs_f32() / 0.15
                } else {
                    1.0 - self.inner.release_start_at.elapsed().as_secs_f32() / 0.1
                }
                .clamp(0.0, 1.0);
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.0;
                1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
            };
        let bottom_left = tr.transform_point(&(Point2::new(-0.5, 0.5) * size));
        let top_right = tr.transform_point(&(Point2::new(0.5, -0.5) * size));
        let height = tr.transform_vector(&Vector2::new(0.0, 1.0)).norm();
        let path = {
            let mut builder = Path::builder();
            builder.add_rounded_rectangle(
                &Box2D::new(
                    (bottom_left.x, bottom_left.y).into(),
                    (top_right.x, top_right.y).into(),
                ),
                &BorderRadii::new(self.radius * height),
                Winding::Positive,
            );
            builder.build()
        };
        target.fill_path(&path, self.color.into_shading(), 1.0);
    }
}
