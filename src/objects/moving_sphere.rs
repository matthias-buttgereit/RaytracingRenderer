use std::{f64::consts::PI, rc::Rc};

use crate::{
    hits::{
        aabb::{surrounding_box, AABB},
        hittable::{HitRecord, Hittable},
    },
    materials::Material,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

#[derive(Clone)]
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
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().len_squared();
        let half_b = dot(&oc, &ray.direction());
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
        let p = ray.at(t);
        let normal = (p - self.center(ray.time())) / self.radius;
        let uv = get_sphere_uv(&normal);

        let mut result = HitRecord {
            t,
            p,
            normal,
            front_face: true,
            surface_coordinates: uv,
            material: Rc::clone(&self.material),
        };
        result.set_face_normal(ray, normal);

        Some(result)
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        let (time0, time1) = time;
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let output_box = surrounding_box(box0, box1);

        Some(output_box)
    }
}

fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    let theta = (-p.y()).acos();
    let phi = (-p.y()).atan2(p.x()) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;

    (u, v)
}
