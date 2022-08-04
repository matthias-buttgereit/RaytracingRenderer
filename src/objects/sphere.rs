use std::{f64::consts::PI, rc::Rc};

use crate::{
    hits::{
        aabb::AABB,
        hittable::{HitRecord, Hittable},
    },
    materials::Material,
    ray,
    vec3::{dot, Point3, Vec3},
};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material: Rc::clone(&material),
        }
    }

    fn get_sphere_uv(&self, p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
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
        let uv = self.get_sphere_uv(&normal);

        let mut result = HitRecord {
            t,
            p,
            normal,
            front_face: true,
            surface_coordinates: uv,
            material: Rc::clone(&self.material),
        };
        result.set_face_normal(r, normal);

        Some(result)
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        let bounding_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(bounding_box)
    }
}
