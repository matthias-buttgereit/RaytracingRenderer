use crate::{
    hits::hittable::HitRecord,
    ray::Ray,
    vec3::{refract, unit_vector, Color},
};

use super::Material;

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, hitrecord: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if hitrecord.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let refracted = refract(&unit_direction, &hitrecord.normal, refraction_ratio);
        Some((Ray::new(hitrecord.p, refracted), Color::new(1.0, 1.0, 1.0)))
    }
}
