pub mod lambertian;
pub mod metal;

use crate::{hits::hittable::HitRecord, ray::Ray, vec3::Color};

pub trait Material {
    fn scatter(&self, r_in: Ray, hitrecord: &HitRecord) -> (Ray, Color);
}
