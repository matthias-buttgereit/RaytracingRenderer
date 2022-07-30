use crate::ray::Ray;

use super::{
    aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
};

#[derive(Default)]
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

    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        if self.list.is_empty() {
            return None;
        }

        let mut return_option = AABB::default();
        let mut first_box = true;

        for object in &self.list {
            match object.bounding_box(time) {
                Some(temp_box) => {
                    if first_box {
                        return_option = temp_box;
                        first_box = false
                    } else {
                        return_option = surrounding_box(temp_box, return_option);
                    }
                }
                None => return None,
            }
        }

        Some(return_option)
    }
}
