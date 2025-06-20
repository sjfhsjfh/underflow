use comui::utils::Transform;
use lyon::geom::{Point, Vector, traits::Transformation};

#[allow(dead_code)]
pub struct UTransform {
    inner: Transform,
}

#[allow(dead_code)]
impl UTransform {
    pub fn new(inner: Transform) -> Self {
        Self { inner }
    }

    pub fn identity() -> Self {
        Self {
            inner: Transform::identity(),
        }
    }
}

impl Transformation<f32> for UTransform {
    fn transform_point(&self, p: Point<f32>) -> Point<f32> {
        let p = nalgebra::Point2::new(p.x, p.y);
        let p = self.inner.transform_point(&p);
        lyon::geom::Point::new(p.x, p.y)
    }

    fn transform_vector(&self, v: Vector<f32>) -> Vector<f32> {
        let v = nalgebra::Vector2::new(v.x, v.y);
        let v = self.inner.transform_vector(&v);
        lyon::geom::Vector::new(v.x, v.y)
    }
}
