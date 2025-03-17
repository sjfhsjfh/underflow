use crate::ui::Color;

pub trait Tweenable {
    fn tween(&self, other: &Self, t: f32) -> Self;
}

impl Tweenable for f32 {
    fn tween(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Tweenable for Color {
    fn tween(&self, other: &Self, t: f32) -> Self {
        Color {
            r: self.r.tween(&other.r, t),
            g: self.g.tween(&other.g, t),
            b: self.b.tween(&other.b, t),
            a: self.a.tween(&other.a, t),
        }
    }
}
