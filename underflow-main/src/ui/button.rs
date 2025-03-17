use std::borrow::Cow;

use lyon::path::{Path, Winding};
use macroquad::{
    input::{Touch, TouchPhase},
    math::{Rect, Vec2, Vec4Swizzles, vec4},
    prelude::debug,
};

use crate::utils::{nalgebra_to_glm, rect_to_euclid, semi_black};

use super::{Matrix, Ui, Vector, colors::WHITE};

#[derive(Clone, Copy)]
pub struct RectButton {
    pts: Option<[Vec2; 4]>,
    id: Option<u64>,
}

impl Default for RectButton {
    fn default() -> Self {
        Self::new()
    }
}

impl RectButton {
    pub fn new() -> Self {
        Self {
            pts: None,
            id: None,
        }
    }

    pub fn touching(&self) -> bool {
        self.id.is_some()
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        if let Some([a, b, c, d]) = self.pts {
            let abp = (b - a).perp_dot(pos - a);
            let bcp = (c - b).perp_dot(pos - b);
            let cdp = (d - c).perp_dot(pos - c);
            let dap = (a - d).perp_dot(pos - d);
            (abp >= 0. && bcp >= 0. && cdp >= 0. && dap >= 0.)
                || (abp <= 0. && bcp <= 0. && cdp <= 0. && dap <= 0.)
        } else {
            false
        }
    }

    pub fn set(&mut self, ui: &mut Ui, rect: Rect) {
        let mat = nalgebra_to_glm(&ui.transform) * ui.gl_transform;
        let tr = |x: f32, y: f32| {
            let pos = mat * vec4(x, y, 0., 1.);
            pos.xy() / pos.w
        };
        self.pts = Some([
            tr(rect.x, rect.y),
            tr(rect.right(), rect.y),
            tr(rect.right(), rect.bottom()),
            tr(rect.x, rect.bottom()),
        ]);
    }

    pub fn touch(&mut self, touch: &Touch) -> bool {
        let inside = self.contains(touch.position);
        match touch.phase {
            TouchPhase::Started => {
                if inside {
                    self.id = Some(touch.id);
                }
            }
            TouchPhase::Moved | TouchPhase::Stationary => {
                if self.id == Some(touch.id) && !inside {
                    self.id = None;
                }
            }
            TouchPhase::Cancelled => {
                self.id = None;
            }
            TouchPhase::Ended => {
                if self.id.take() == Some(touch.id) && inside {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone)]
pub struct DRectButton {
    pub inner: RectButton,
    last_touching: bool,
    start_time: Option<f32>,
    delta: f32,
}
impl Default for DRectButton {
    fn default() -> Self {
        Self::new()
    }
}
impl DRectButton {
    pub const TIME: f32 = 0.2;

    pub fn new() -> Self {
        Self {
            inner: RectButton::new(),
            last_touching: false,
            start_time: None,
            delta: -0.006,
        }
    }

    pub fn build(&mut self, ui: &mut Ui, t: f32, r: Rect, f: impl FnOnce(&mut Ui, Path)) {
        self.inner.set(ui, r);
        // let r = r.feather((1. - self.progress(t)) * self.delta);
        let ct = r.center();
        let ct = Vector::new(ct.x, ct.y);
        let mut path = Path::builder();
        path.add_rectangle(&rect_to_euclid(&r), Winding::Positive);
        let r = path.build();
        ui.with(
            Matrix::new_translation(&-ct)
                .append_scaling(1. - (1. - self.progress(t)) * 0.04)
                .append_translation(&ct),
            |ui| {
                f(ui, r);
            },
        );
    }

    pub fn invalidate(&mut self) {
        self.inner.pts = None;
    }

    // pub fn render_text<'a>(
    //     &mut self,
    //     ui: &mut Ui,
    //     r: Rect,
    //     t: f32,
    //     text: impl Into<Cow<'a, str>>,
    //     size: f32,
    //     chosen: bool,
    // ) {
    //     let oh = r.h;
    //     self.build(ui, t, r, |ui, path| {
    //         let ct = r.center();
    //         ui.fill_path(&path, if chosen { WHITE } else { semi_black(0.4) });
    //         ui.text(text)
    //             .pos(ct.x, ct.y)
    //             .anchor(0.5, 0.5)
    //             .no_baseline()
    //             .size(size * (1. - (1. - r.h / oh).powf(1.3)))
    //             .max_width(r.w)
    //             .color(if chosen {
    //                 Color::new(0.3, 0.3, 0.3, 1.)
    //             } else {
    //                 WHITE
    //             })
    //             .draw();
    //     });
    // }

    // pub fn render_text_left<'a>(
    //     &mut self,
    //     ui: &mut Ui,
    //     r: Rect,
    //     t: f32,
    //     alpha: f32,
    //     text: impl Into<Cow<'a, str>>,
    //     size: f32,
    //     chosen: bool,
    // ) {
    //     let oh = r.h;
    //     self.build(ui, t, r, |ui, path| {
    //         ui.fill_path(&path, if chosen { WHITE } else { semi_black(0.4) });
    //         ui.text(text)
    //             .pos(r.x + 0.02, r.center().y)
    //             .anchor(0., 0.5)
    //             .max_width(r.w - 0.04)
    //             .no_baseline()
    //             .size(size * r.h / oh)
    //             .color(if chosen {
    //                 Color::new(0.3, 0.3, 0.3, alpha)
    //             } else {
    //                 semi_white(alpha)
    //             })
    //             .draw();
    //     });
    // }

    // #[inline]
    // pub fn render_input<'a>(
    //     &mut self,
    //     ui: &mut Ui,
    //     r: Rect,
    //     t: f32,
    //     text: impl Into<Cow<'a, str>>,
    //     hint: impl Into<Cow<'a, str>>,
    //     size: f32,
    // ) {
    //     let text = text.into();
    //     if text.trim().is_empty() {
    //         self.render_text_left(ui, r, t, 0.7, hint, size, false);
    //     } else {
    //         self.render_text_left(ui, r, t, 1., text, size, false);
    //     }
    // }

    #[inline]
    pub fn with_delta(mut self, delta: f32) -> Self {
        self.delta = delta;
        self
    }

    pub fn progress(&mut self, t: f32) -> f32 {
        if self
            .start_time
            .as_ref()
            .map_or(false, |it| t > *it + Self::TIME)
        {
            self.start_time = None;
        }
        let p = if let Some(time) = &self.start_time {
            (t - time) / Self::TIME
        } else {
            1.
        };
        if self.inner.touching() { 1. - p } else { p }
    }

    pub fn touch(&mut self, touch: &Touch, t: f32) -> bool {
        let res = self.inner.touch(touch);
        let touching = self.inner.touching();
        if self.last_touching != touching {
            self.last_touching = touching;
            self.start_time = Some(t);
        }
        // if res && self.play_sound {
        //     button_hit();
        // }
        res
    }
}
