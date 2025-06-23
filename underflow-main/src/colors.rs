pub use macroquad::color::*;

pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}

pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    rgba(r, g, b, 255)
}

pub fn color_primary() -> Color {
    rgb(103, 80, 164)
}

pub fn color_secondary() -> Color {
    rgb(98, 91, 113)
}

pub fn color_secondary_container() -> Color {
    rgb(232, 222, 248)
}

pub fn color_tertiary() -> Color {
    rgb(125, 82, 96)
}
