use std::rc::Rc;

use crate::{
    hits::hittable::HitRecord,
    ray::Ray,
    textures::{solid_color::SolidColor, Texture},
    vec3::{random_unit_vector, Color},
};

use super::Material;

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new_from_color(albedo)),
        }
    }

    pub fn new_from_texture(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    #[allow(unused_variables)]
    fn scatter(&self, r_in: Ray, hitrecord: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hitrecord.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hitrecord.normal;
        }

        let scattered_ray = Ray::new(hitrecord.p, scatter_direction, r_in.time());
        let attenuation = self
            .albedo
            .value(hitrecord.surface_coordinates, &hitrecord.p);
        Some((scattered_ray, attenuation))
    }
}
