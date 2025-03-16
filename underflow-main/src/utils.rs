use lyon::math::Box2D;
use macroquad::{
    input::Touch,
    math::{Mat4, Rect},
};

use crate::{
    Ui,
    ui::{Color, Matrix},
};

pub fn nalgebra_to_glm(mat: &Matrix) -> Mat4 {
    /*
        [11] [12]  0  [13]
        [21] [22]  0  [23]
          0    0   1    0
        [31] [32]  0  [33]
    */
    Mat4::from_cols_array(&[
        mat.m11, mat.m21, 0., mat.m31, mat.m12, mat.m22, 0., mat.m32, 0., 0., 1., 0., mat.m13,
        mat.m23, 0., mat.m33,
    ])
}

#[inline]
pub(crate) fn semi_black(alpha: f32) -> Color {
    Color::new(0., 0., 0., alpha)
}

pub(crate) fn screen_to_world(ui: &Ui, touch: &Touch) -> Touch {
    Touch {
        position: ui.camera().screen_to_world(touch.position),
        ..*touch
    }
}

pub fn rect_to_euclid(r: &Rect) -> Box2D {
    Box2D::new(
        lyon::math::point(r.x, r.y),
        lyon::math::point(r.right(), r.bottom()),
    )
}
