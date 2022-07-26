use crate::{
    ray,
    vec3::{dot, Point3}, hits::hittable::{Hittable, HitRecord},
};

#[derive(Default, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
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
        let normal = (p - self.center) / self.radius;

        let mut result = HitRecord {
            t,
            p,
            normal,
            front_face: false,
        };
        result.set_face_normal(r, normal);

        Some(result)
    }
}
