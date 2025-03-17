use lyon::tessellation::{
    FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor,
};
use macroquad::{texture::Texture2D, ui::Vertex};

use crate::tween::Tweenable;

use super::{Color, Matrix, Point};

pub trait Shading {
    fn new_vertex(&self, mat: &Matrix, p: &Point, alpha: f32) -> Vertex;
    fn texture(&self) -> Option<Texture2D>;
}
pub struct GradientShading {
    origin: (f32, f32),
    color: Color,
    vector: (f32, f32),
    color_end: Color,
}

impl Shading for GradientShading {
    fn new_vertex(&self, mat: &Matrix, p: &Point, alpha: f32) -> Vertex {
        let t = mat.transform_point(p);
        let mut color = {
            let (dx, dy) = (p.x - self.origin.0, p.y - self.origin.1);
            Color::tween(
                &self.color,
                &self.color_end,
                dx * self.vector.0 + dy * self.vector.1,
            )
        };
        color.a *= alpha;

        Vertex::new(t.x, t.y, 0., 0., 0., color.into())
    }

    fn texture(&self) -> Option<Texture2D> {
        None
    }
}

pub trait IntoShading {
    type Target: Shading;

    fn into_shading(self) -> Self::Target;
}

impl<T: Shading> IntoShading for T {
    type Target = T;

    fn into_shading(self) -> Self::Target {
        self
    }
}

impl IntoShading for Color {
    type Target = GradientShading;

    fn into_shading(self) -> Self::Target {
        GradientShading {
            origin: (0., 0.),
            color: self,
            vector: (1., 0.),
            color_end: self,
        }
    }
}

pub(super) struct ShadedConstructor<T: Shading>(pub(super) Matrix, pub T, pub(super) f32);
impl<T: Shading> FillVertexConstructor<Vertex> for ShadedConstructor<T> {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        let pos = vertex.position();
        self.1
            .new_vertex(&self.0, &Point::new(pos.x, pos.y), self.2)
    }
}
impl<T: Shading> StrokeVertexConstructor<Vertex> for ShadedConstructor<T> {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        let pos = vertex.position();
        self.1
            .new_vertex(&self.0, &Point::new(pos.x, pos.y), self.2)
    }
}
