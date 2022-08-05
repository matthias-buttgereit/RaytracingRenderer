pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

use crate::{
    hits::hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Point3},
};

pub trait Material {
    fn scatter(&self, r_in: Ray, hitrecord: &HitRecord) -> Option<(Ray, Color)>;

    #[allow(unused_variables)]
    fn emitted(&self, uv: (f64, f64), p: &Point3) -> Color {
        Color::default()
    }
}
