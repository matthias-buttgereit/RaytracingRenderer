use std::rc::Rc;

use crate::{
    hits::hittable::{HitRecord, Hittable},
    materials::Material,
    ray::Ray,
    vec3::{dot, Point3},
};

pub struct MovingSphere {
    centers: (Point3, Point3),
    time_frame: (f64, f64),
    radius: f64,
    material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        centers: (Point3, Point3),
        time_frame: (f64, f64),
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            centers,
            time_frame,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        let (center0, center1) = self.centers;
        let (time0, time1) = self.time_frame;
        center0 + ((time - time0) / (time1 - time0)) * (center1 - center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().len_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.len_squared() - self.radius * self.radius;

        let discrim = half_b * half_b - a * c;

        if discrim < 0.0 {
            return None;
        }

        let sqrtd = discrim.sqrt();
        let (t_min, t_max) = interval;
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let normal = (p - self.center(r.time())) / self.radius;

        let mut result = HitRecord {
            t,
            p,
            normal,
            front_face: true,
            material: Rc::clone(&self.material),
        };
        result.set_face_normal(r, normal);

        Some(result)
    }
}
