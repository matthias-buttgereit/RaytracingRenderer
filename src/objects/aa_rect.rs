use std::rc::Rc;

use crate::{
    hits::{
        aabb::AABB,
        hittable::{HitRecord, Hittable},
    },
    materials::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct XYRect {
    material: Rc<dyn Material>,
    x_boundaries: (f64, f64),
    y_boundaries: (f64, f64),
    k: f64,
}

impl XYRect {
    pub fn new(x: (f64, f64), y: (f64, f64), k: f64, material: Rc<dyn Material>) -> Self {
        Self {
            material,
            x_boundaries: x,
            y_boundaries: y,
            k,
        }
    }
}

impl Hittable for XYRect {
    #[allow(unused_variables)]
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x_boundaries.0, self.y_boundaries.0, self.k - 0.0001),
            Point3::new(self.x_boundaries.1, self.y_boundaries.1, self.k + 0.0001),
        ))
    }

    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();

        if t < interval.0 || t > interval.1 {
            return None;
        }

        let x_value = r.origin().x() + t * r.direction().x();
        let y_value = r.origin().y() + t * r.direction().y();

        if x_value < self.x_boundaries.0
            || x_value > self.x_boundaries.1
            || y_value < self.y_boundaries.0
            || y_value > self.y_boundaries.1
        {
            return None;
        }

        let u = (x_value - self.x_boundaries.0) / (self.x_boundaries.1 - self.x_boundaries.0);
        let v = (y_value - self.y_boundaries.0) / (self.y_boundaries.1 - self.y_boundaries.0);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let uv = (u, v);
        let p = r.at(t);

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
}

pub struct XZRect {
    material: Rc<dyn Material>,
    x_boundaries: (f64, f64),
    z_boundaries: (f64, f64),
    k: f64,
}

impl XZRect {
    pub fn new(x: (f64, f64), z: (f64, f64), k: f64, material: Rc<dyn Material>) -> Self {
        Self {
            material,
            x_boundaries: x,
            z_boundaries: z,
            k,
        }
    }
}

impl Hittable for XZRect {
    #[allow(unused_variables)]
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x_boundaries.0, self.k - 0.0001, self.z_boundaries.0),
            Point3::new(self.x_boundaries.1, self.k + 0.0001, self.z_boundaries.1),
        ))
    }

    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();

        if t < interval.0 || t > interval.1 {
            return None;
        }

        let x_value = r.origin().x() + t * r.direction().x();
        let z_value = r.origin().z() + t * r.direction().z();

        if x_value < self.x_boundaries.0
            || x_value > self.x_boundaries.1
            || z_value < self.z_boundaries.0
            || z_value > self.z_boundaries.1
        {
            return None;
        }

        let u = (x_value - self.x_boundaries.0) / (self.x_boundaries.1 - self.x_boundaries.0);
        let v = (z_value - self.z_boundaries.0) / (self.z_boundaries.1 - self.z_boundaries.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let uv = (u, v);
        let p = r.at(t);

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
}

pub struct YZRect {
    material: Rc<dyn Material>,
    y_boundaries: (f64, f64),
    z_boundaries: (f64, f64),
    k: f64,
}

impl YZRect {
    pub fn new(y: (f64, f64), z: (f64, f64), k: f64, material: Rc<dyn Material>) -> Self {
        Self {
            material,
            y_boundaries: y,
            z_boundaries: z,
            k,
        }
    }
}

impl Hittable for YZRect {
    #[allow(unused_variables)]
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.k - 0.0001, self.y_boundaries.0, self.z_boundaries.0),
            Point3::new(self.k + 0.0001, self.y_boundaries.1, self.z_boundaries.1),
        ))
    }

    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();

        if t < interval.0 || t > interval.1 {
            return None;
        }

        let y_value = r.origin().y() + t * r.direction().y();
        let z_value = r.origin().z() + t * r.direction().z();

        if y_value < self.y_boundaries.0
            || y_value > self.y_boundaries.1
            || z_value < self.z_boundaries.0
            || z_value > self.z_boundaries.1
        {
            return None;
        }

        let u = (y_value - self.y_boundaries.0) / (self.y_boundaries.1 - self.y_boundaries.0);
        let v = (z_value - self.z_boundaries.0) / (self.z_boundaries.1 - self.z_boundaries.0);
        let normal = Vec3::new(1.0, 0.0, 0.0);
        let uv = (u, v);
        let p = r.at(t);

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
}
