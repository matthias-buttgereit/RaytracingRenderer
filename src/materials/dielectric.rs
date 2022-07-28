use rand::{thread_rng, Rng};

use crate::{
    hits::hittable::HitRecord,
    ray::Ray,
    vec3::{dot, reflect, refract, unit_vector, Color},
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

        let cos_theta = dot(&-unit_direction, &hitrecord.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > thread_rng().gen_range(0.0..1.0)
        {
            reflect(&unit_direction, &hitrecord.normal)
        } else {
            refract(&unit_direction, &hitrecord.normal, refraction_ratio)
        };

        Some((Ray::new(hitrecord.p, direction), Color::new(1.0, 1.0, 1.0)))
    }
}

fn reflectance(cosine: f64, ref_index: f64) -> f64 {
    let r0 = ((1.0 - ref_index) / (1.0 + ref_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
