use crate::{ray::Ray, vec3::Vec3};

use super::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
};

pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Box<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            object: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        let min = self.object.bounding_box(time).unwrap().min() + self.offset;
        let max = self.object.bounding_box(time).unwrap().max() + self.offset;
        Some(AABB::new(min, max))
    }

    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        match self.object.hit(&moved_r, interval) {
            Some(mut hitrecord) => {
                hitrecord.p += self.offset;
                hitrecord.set_face_normal(&moved_r, hitrecord.normal);
                Some(hitrecord)
            }
            None => None,
        }
    }
}
