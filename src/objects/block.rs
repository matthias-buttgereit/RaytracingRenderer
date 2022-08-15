use std::rc::Rc;

use crate::{
    hits::{
        aabb::AABB,
        hittable::{HitRecord, Hittable},
        hittalbe_list::HittableList,
    },
    materials::Material,
    ray::Ray,
    vec3::Point3,
};

use super::aa_rect::{XYRect, XZRect, YZRect};

pub struct Block {
    block_min: Point3,
    block_max: Point3,
    sides: HittableList,
}

impl Block {
    pub fn new(p0: Point3, p1: Point3, material: Rc<dyn Material>) -> Self {
        let mut new = Self {
            block_min: p0,
            block_max: p1,
            sides: HittableList::new(),
        };

        new.sides.add(Box::new(XYRect::new(
            (p0.x(), p1.x()),
            (p0.y(), p1.y()),
            p1.z(),
            material.clone(),
        )));
        new.sides.add(Box::new(XYRect::new(
            (p0.x(), p1.x()),
            (p0.y(), p1.y()),
            p0.z(),
            material.clone(),
        )));

        new.sides.add(Box::new(XZRect::new(
            (p0.x(), p1.x()),
            (p0.z(), p1.z()),
            p1.y(),
            material.clone(),
        )));
        new.sides.add(Box::new(XZRect::new(
            (p0.x(), p1.x()),
            (p0.z(), p1.z()),
            p0.y(),
            material.clone(),
        )));

        new.sides.add(Box::new(YZRect::new(
            (p0.y(), p1.y()),
            (p0.z(), p1.z()),
            p1.x(),
            material.clone(),
        )));
        new.sides.add(Box::new(YZRect::new(
            (p0.y(), p1.y()),
            (p0.z(), p1.z()),
            p0.x(),
            material,
        )));

        new
    }
}

impl Hittable for Block {
    #[allow(unused_variables)]
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        Some(AABB::new(self.block_min, self.block_max))
    }

    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        self.sides.hit(r, interval)
    }
}
