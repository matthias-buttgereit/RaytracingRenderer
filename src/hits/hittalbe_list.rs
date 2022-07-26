use crate::ray::Ray;

use super::hittable::{HitRecord, Hittable};

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn add(&mut self, value: Box<dyn Hittable>) {
        self.list.push(value)
    }

    pub fn new() -> Self {
        Self { list: vec![] }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let (t_min, mut closest_so_far) = interval;

        for object in &self.list {
            if let Some(hit) = object.hit(r, (t_min, closest_so_far)) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            };
        }
        hit_anything
    }
}
