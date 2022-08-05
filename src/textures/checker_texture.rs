use std::rc::Rc;

use crate::vec3::Color;

use super::{solid_color::SolidColor, Texture};

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn new_from_color(c1: Color, c2: Color) -> Self {
        Self::new(
            Rc::new(SolidColor::new_from_color(c1)),
            Rc::new(SolidColor::new_from_color(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: (f64, f64), p: &crate::vec3::Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(uv, p)
        } else {
            self.even.value(uv, p)
        }
    }
}
