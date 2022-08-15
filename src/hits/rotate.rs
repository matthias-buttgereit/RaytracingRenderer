use crate::{
    degrees_to_radians,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
};

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    b_box: Option<AABB>,
}

impl RotateY {
    pub fn new(object: Box<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let bounding_box = object.bounding_box((0.0, 1.0));
        let mut new = Self {
            object,
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
            b_box: bounding_box,
        };

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let aabb = new.b_box.unwrap();
                    let x = f64::from(i) * aabb.max().x() + f64::from(1 - i) * aabb.min().x();
                    let y = f64::from(j) * aabb.max().y() + f64::from(1 - j) * aabb.min().y();
                    let z = f64::from(k) * aabb.max().z() + f64::from(1 - k) * aabb.min().z();

                    let new_x = new.cos_theta * x + new.sin_theta * z;
                    let new_z = -new.sin_theta * x + new.cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        new.b_box = Some(AABB::new(min, max));
        new
    }
}

impl Hittable for RotateY {
    #[allow(unused_variables)]
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        self.b_box
    }

    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        match self.object.hit(&rotated_r, interval) {
            Some(mut hitrecord) => {
                let mut p = hitrecord.p;
                let mut normal = hitrecord.normal;

                p[0] = self.cos_theta * hitrecord.p[0] + self.sin_theta * hitrecord.p[2];
                p[2] = -self.sin_theta * hitrecord.p[0] + self.cos_theta * hitrecord.p[2];

                normal[0] =
                    self.cos_theta * hitrecord.normal[0] + self.sin_theta * hitrecord.normal[2];
                normal[2] =
                    -self.sin_theta * hitrecord.normal[0] + self.cos_theta * hitrecord.normal[2];

                hitrecord.p = p;
                hitrecord.set_face_normal(&rotated_r, normal);

                Some(hitrecord)
            }
            None => None,
        }
    }
}
