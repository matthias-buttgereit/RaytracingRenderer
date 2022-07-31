use crate::vec3::{Color, Point3};

pub trait Texture {
    fn value(&self, uv: (f64, f64), p: &Point3) -> Color;
}
