pub(crate) mod button;
pub(crate) mod shading;

use lyon::{
    path::PathEvent,
    tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
        VertexBuffers,
    },
};
use macroquad::{
    camera::Camera2D,
    math::{Mat4, Rect, vec2},
    miniquad::PassAction,
    prelude::DrawMode,
    texture::Texture2D,
    ui::Vertex,
    window::{get_internal_gl, screen_height, screen_width},
};
use shading::{IntoShading, ShadedConstructor, Shading};

pub type Point = nalgebra::Point2<f32>;
pub type Vector = nalgebra::Vector2<f32>;
pub type Matrix = nalgebra::Matrix3<f32>;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Color> for macroquad::color::Color {
    fn from(c: Color) -> Self {
        macroquad::color::Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

const fn color_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color {
        r: r as f32 / 255.,
        g: g as f32 / 255.,
        b: b as f32 / 255.,
        a: a as f32 / 255.,
    }
}

pub mod colors {
    //! Constants for some common colors.

    use super::Color;

    pub const LIGHTGRAY: Color = Color::new(0.78, 0.78, 0.78, 1.00);
    pub const GRAY: Color = Color::new(0.51, 0.51, 0.51, 1.00);
    pub const DARKGRAY: Color = Color::new(0.31, 0.31, 0.31, 1.00);
    pub const YELLOW: Color = Color::new(0.99, 0.98, 0.00, 1.00);
    pub const GOLD: Color = Color::new(1.00, 0.80, 0.00, 1.00);
    pub const ORANGE: Color = Color::new(1.00, 0.63, 0.00, 1.00);
    pub const PINK: Color = Color::new(1.00, 0.43, 0.76, 1.00);
    pub const RED: Color = Color::new(0.90, 0.16, 0.22, 1.00);
    pub const MAROON: Color = Color::new(0.75, 0.13, 0.22, 1.00);
    pub const GREEN: Color = Color::new(0.00, 0.89, 0.19, 1.00);
    pub const LIME: Color = Color::new(0.00, 0.62, 0.18, 1.00);
    pub const DARKGREEN: Color = Color::new(0.00, 0.46, 0.17, 1.00);
    pub const SKYBLUE: Color = Color::new(0.40, 0.75, 1.00, 1.00);
    pub const BLUE: Color = Color::new(0.00, 0.47, 0.95, 1.00);
    pub const DARKBLUE: Color = Color::new(0.00, 0.32, 0.67, 1.00);
    pub const PURPLE: Color = Color::new(0.78, 0.48, 1.00, 1.00);
    pub const VIOLET: Color = Color::new(0.53, 0.24, 0.75, 1.00);
    pub const DARKPURPLE: Color = Color::new(0.44, 0.12, 0.49, 1.00);
    pub const BEIGE: Color = Color::new(0.83, 0.69, 0.51, 1.00);
    pub const BROWN: Color = Color::new(0.50, 0.42, 0.31, 1.00);
    pub const DARKBROWN: Color = Color::new(0.30, 0.25, 0.18, 1.00);
    pub const WHITE: Color = Color::new(1.00, 1.00, 1.00, 1.00);
    pub const BLACK: Color = Color::new(0.00, 0.00, 0.00, 1.00);
    pub const BLANK: Color = Color::new(0.00, 0.00, 0.00, 0.00);
    pub const MAGENTA: Color = Color::new(1.00, 0.00, 1.00, 1.00);
}

pub struct VertexBuilder<T: Shading> {
    matrix: Matrix,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    shading: T,
    alpha: f32,
}
impl<T: Shading> VertexBuilder<T> {
    fn new(matrix: Matrix, shading: T, alpha: f32) -> Self {
        Self {
            matrix,
            vertices: Vec::new(),
            indices: Vec::new(),
            shading,
            alpha,
        }
    }

    pub fn add(&mut self, x: f32, y: f32) {
        self.vertices.push(
            self.shading
                .new_vertex(&self.matrix, &Point::new(x, y), self.alpha),
        );
    }

    pub fn triangle(&mut self, x: u16, y: u16, z: u16) {
        self.indices.push(x);
        self.indices.push(y);
        self.indices.push(z);
    }

    pub fn commit(&self) {
        let gl = unsafe { get_internal_gl() }.quad_gl;
        gl.texture(self.shading.texture().as_ref());
        gl.draw_mode(DrawMode::Triangles);
        gl.geometry(&self.vertices, &self.indices);
    }
}

pub(crate) struct Ui {
    pub viewport: (i32, i32, i32, i32),
    pub transform: Matrix,
    pub alpha: f32,
    pub gl_transform: Mat4,
    vertex_buffers: VertexBuffers<Vertex, u16>,
    fill_tess: FillTessellator,
    fill_options: FillOptions,
    stroke_tess: StrokeTessellator,
    pub stroke_options: StrokeOptions,
}

impl Ui {
    pub fn new(viewport: Option<(i32, i32, i32, i32)>) -> Self {
        unsafe { get_internal_gl() }
            .quad_context
            .begin_default_pass(PassAction::Clear {
                depth: None,
                stencil: Some(0),
                color: None,
            });
        let viewport =
            viewport.unwrap_or_else(|| (0, 0, screen_width() as i32, screen_height() as i32));
        Self {
            viewport,

            transform: Matrix::identity(),
            gl_transform: Mat4::IDENTITY,

            vertex_buffers: VertexBuffers::new(),
            fill_tess: FillTessellator::new(),
            fill_options: FillOptions::default(),
            stroke_tess: StrokeTessellator::new(),
            stroke_options: StrokeOptions::default(),

            alpha: 1.,
        }
    }

    pub fn camera(&self) -> Camera2D {
        Camera2D {
            zoom: vec2(1., -self.viewport.2 as f32 / self.viewport.3 as f32),
            viewport: Some(self.viewport),
            ..Default::default()
        }
    }

    pub fn fill_rect(&mut self, rect: Rect, shading: impl IntoShading) {
        let mut b = self.builder(shading);
        b.add(rect.x, rect.y);
        b.add(rect.x + rect.w, rect.y);
        b.add(rect.x, rect.y + rect.h);
        b.add(rect.x + rect.w, rect.y + rect.h);
        b.triangle(0, 1, 2);
        b.triangle(1, 2, 3);
        b.commit();
    }

    pub fn fill_path(
        &mut self,
        path: impl IntoIterator<Item = PathEvent>,
        shading: impl IntoShading,
    ) {
        self.draw_lyon(shading.into_shading(), |this, shaded| {
            this.fill_tess
                .tessellate(
                    path,
                    &this.fill_options,
                    &mut BuffersBuilder::new(&mut this.vertex_buffers, shaded),
                )
                .unwrap();
        });
    }

    pub fn fill_circle(&mut self, x: f32, y: f32, radius: f32, shading: impl IntoShading) {
        self.draw_lyon(shading.into_shading(), |this, shaded| {
            this.fill_tess
                .tessellate_circle(
                    lyon::math::point(x, y),
                    radius,
                    &this.fill_options,
                    &mut BuffersBuilder::new(&mut this.vertex_buffers, shaded),
                )
                .unwrap();
        });
    }

    pub fn builder<T: IntoShading>(&self, shading: T) -> VertexBuilder<T::Target> {
        VertexBuilder::new(self.transform, shading.into_shading(), self.alpha)
    }

    #[inline]
    pub fn with<R>(&mut self, transform: Matrix, f: impl FnOnce(&mut Self) -> R) -> R {
        let old = self.transform;
        self.transform = old * transform;
        let res = f(self);
        self.transform = old;
        res
    }

    fn set_tolerance(&mut self) {
        let tol = 0.15
            / (self.transform.transform_vector(&Vector::new(1., 0.)).norm() * screen_width() / 2.);
        self.fill_options.tolerance = tol;
        self.stroke_options.tolerance = tol;
    }

    fn draw_lyon<T: Shading>(
        &mut self,
        shading: T,
        f: impl FnOnce(&mut Self, ShadedConstructor<T>),
    ) {
        self.set_tolerance();
        let shaded = ShadedConstructor(self.transform, shading.into_shading(), self.alpha);
        let tex = shaded.1.texture();
        f(self, shaded);
        self.emit_lyon(tex);
    }

    fn emit_lyon(&mut self, texture: Option<Texture2D>) {
        let gl = unsafe { get_internal_gl() }.quad_gl;
        gl.texture(texture.as_ref());
        gl.draw_mode(DrawMode::Triangles);
        gl.geometry(
            &std::mem::take(&mut self.vertex_buffers.vertices),
            &std::mem::take(&mut self.vertex_buffers.indices),
        );
    }
}
