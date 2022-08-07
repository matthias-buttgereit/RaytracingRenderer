use std::rc::Rc;

use crate::{
    hits::hittable::HitRecord,
    ray::Ray,
    textures::{solid_color::SolidColor, Texture},
    vec3::{random_in_unit_sphere, Color},
};

use super::Material;

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new_from_color(c: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new_from_color(c)),
        }
    }

    pub fn new(a: Rc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: Ray, hitrecord: &HitRecord) -> Option<(Ray, Color)> {
        let scattered = Ray::new(hitrecord.p, random_in_unit_sphere(), r_in.time());
        let attenuation = self
            .albedo
            .value(hitrecord.surface_coordinates, &hitrecord.p);
        Some((scattered, attenuation))
    }
}
