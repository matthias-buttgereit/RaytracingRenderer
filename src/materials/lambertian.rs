use crate::{
    hits::hittable::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, Color},
};

use super::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
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

        let scattered_ray = Ray::new(hitrecord.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered_ray, attenuation))
    }
}
