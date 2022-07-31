use std::rc::Rc;

use crate::{
    materials::Material,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

use super::aabb::AABB;

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord>;
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB>;
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

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        Self {
            p: self.p,
            normal: self.normal,
            t: self.t,
            front_face: self.front_face,
            material: Rc::clone(&self.material),
        }
    }
}
