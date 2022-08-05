use crate::vec3::Color;

use super::Texture;

#[derive(Debug, Default)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::new(r, g, b),
        }
    }

    pub fn new_from_color(c: Color) -> Self {
        Self { color_value: c }
    }
}

impl Texture for SolidColor {
    #[allow(unused_variables)]
    fn value(&self, uv: (f64, f64), p: &crate::vec3::Point3) -> Color {
        self.color_value
    }
}
