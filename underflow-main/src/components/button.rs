use comui::{
    component::Component,
    components::{button::QuadButton, label::Label},
    layout::{Layout, LayoutBuilder},
    shading::IntoShading,
    utils::Transform,
    window::Window,
};
use lyon::{math::Point, path::Path};
use macroquad::color::Color;

use crate::{
    colors,
    components::rounded_rect::{RoundedRect, RoundedRectBuilder},
    tl,
    utils::UTransform,
};

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
            color: colors::BLACK,
            inner: QuadButton::default(),
        }
    }
}

impl Layout for RoundedButton {
    fn components(&mut self) -> Vec<(Transform, &mut dyn comui::component::Component)> {
        let rect = (0.0, 0.0, 1.0 - 0.5 * self.radius, 1.0);
        LayoutBuilder::new()
            .at_rect(rect, &mut self.inner as &mut dyn Component)
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
        RoundedRect::builder()
            .with_fill_color(self.color)
            .with_radius_rel(0.0, self.radius)
            .build()
            .render(&(tr * Transform::new_scaling(size)), target);
    }
}

#[allow(dead_code)]
pub struct LabeledButton {
    label_component: Label,
    raw_size: f32,
    pub inner: RoundedButton,
    l10n_id: String,
}

impl LabeledButton {
    pub fn back_btn() -> Self {
        Self::new_with_id(
            "back",
            |label| {
                label
                    .with_font_size(48.)
                    .with_line_height(48.)
                    .with_texture_align((0.5, 0.6))
                    .with_color(colors::WHITE)
            },
            |button| {
                button
                    .with_color(colors::color_secondary())
                    .with_radius(0.5)
            },
        )
    }

    pub fn new_with_id(
        id: impl AsRef<str>,
        label_f: impl FnOnce(Label) -> Label,
        button_f: impl FnOnce(RoundedButton) -> RoundedButton,
    ) -> Self {
        let l10n_id = id.as_ref().to_string();
        let label_component = label_f(Label::new(tl!(l10n_id.clone())));
        Self {
            raw_size: label_component.font_size,
            label_component,
            inner: button_f(RoundedButton::default()),
            l10n_id,
        }
    }

    pub fn triggered(&mut self) -> bool {
        if self.inner.inner.triggered {
            self.inner.inner.triggered = false;
            true
        } else {
            false
        }
    }
}

impl Layout for LabeledButton {
    fn before_render(&mut self, _: &Transform, _: &mut Window) {
        self.label_component.text = tl!(self.l10n_id.clone()).into_owned();
        // ! TODO: memory issue here!!!
        // let size = 1.0
        //     - 0.04 * {
        //         let t = if self.inner.inner.pressed {
        //             self.inner.inner.press_start_at.elapsed().as_secs_f32() / 0.15
        //         } else {
        //             1.0 - self.inner.inner.release_start_at.elapsed().as_secs_f32() / 0.1
        //         }
        //         .clamp(0.0, 1.0);
        //         const C1: f32 = 1.70158;
        //         const C3: f32 = C1 + 1.0;
        //         1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
        //     };
        // self.label_component.font_size = self.raw_size * size;
    }

    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect((0.0, 0.0, 1.0, 1.0), &mut self.inner as &mut dyn Component)
            .at_rect(
                (0.0, 0.0, 1.0, 1.0),
                &mut self.label_component as &mut dyn Component,
            )
            .build()
    }
}

pub struct CancelButton {
    container: Option<RoundedRect>,
    inner: QuadButton,
    /// Stroke for the cross
    stroke: (f32, Color),
    size: f32,
}

#[must_use = "Call `build` to finalize the cancel button configuration"]
pub struct CancelButtonBuilder {
    container: Option<RoundedRectBuilder>,
    stroke: (f32, Color),
    size: f32,
}

impl Default for CancelButtonBuilder {
    fn default() -> Self {
        Self {
            container: None,
            stroke: (2.0, colors::BLACK),
            size: 0.3,
        }
    }
}

impl CancelButtonBuilder {
    pub fn build(self) -> CancelButton {
        let container = self.container.map(|b| b.build());
        CancelButton {
            container,
            inner: QuadButton::default(),
            stroke: self.stroke,
            size: self.size,
        }
    }

    pub fn with_container(mut self, container: RoundedRectBuilder) -> Self {
        self.container = Some(container);
        self
    }

    pub fn with_stroke(mut self, stroke: (f32, Color)) -> Self {
        self.stroke = stroke;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl CancelButton {
    pub fn builder() -> CancelButtonBuilder {
        Default::default()
    }

    pub fn canceled(&mut self) -> bool {
        if self.inner.triggered {
            self.inner.triggered = false;
            true
        } else {
            false
        }
    }
}

impl Layout for CancelButton {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect((0.0, 0.0, 1.0, 1.0), &mut self.inner as &mut dyn Component)
            .build()
    }

    fn before_render(&mut self, tr: &Transform, target: &mut Window) {
        if let Some(cont) = self.container.as_mut() {
            cont.render(tr, target);
        }
    }

    fn after_render(&mut self, tr: &Transform, target: &mut Window) {
        let mut builder = Path::builder();
        let origin = Point::origin();
        let vert1 = (0.5 * self.size, 0.5 * self.size).into();
        let vert2 = (-0.5 * self.size, 0.5 * self.size).into();
        builder.begin(origin);
        builder.line_to(vert1);
        builder.line_to(origin);
        builder.line_to(vert2);
        builder.line_to(origin);
        builder.line_to(-vert1);
        builder.line_to(origin);
        builder.line_to(-vert2);
        builder.line_to(origin);
        let path = builder.build().transformed(&UTransform::new(*tr));
        let (thickness, color) = self.stroke;
        target.set_stroke_options(|options| options.with_line_join(lyon::path::LineJoin::Round));
        target.stroke_path(&path, color.into_shading(), 1.0, thickness);
        target.set_stroke_options(|_| Default::default());
    }
}
