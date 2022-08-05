use std::rc::Rc;

use crate::{
    hits::hittable::HitRecord,
    ray::Ray,
    textures::{solid_color::SolidColor, Texture},
    vec3::Color,
};

use super::Material;

pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(c: Color) -> Self {
        Self {
            emit: Rc::new(SolidColor::new_from_color(c)),
        }
    }

    pub fn new_from_texture(a: Rc<dyn Texture>) -> Self {
        Self { emit: a }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, uv: (f64, f64), p: &crate::vec3::Point3) -> Color {
        self.emit.value(uv, p)
    }

    #[allow(unused_variables)]
    fn scatter(&self, r_in: Ray, hitrecord: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}
