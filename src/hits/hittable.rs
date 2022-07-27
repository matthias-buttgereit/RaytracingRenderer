use std::rc::Rc;

use crate::{
    materials::Material,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord>;
}
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
