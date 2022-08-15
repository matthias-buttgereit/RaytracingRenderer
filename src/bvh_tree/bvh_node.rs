use rand::{prelude::SliceRandom, thread_rng, Rng};

use crate::{
    hits::{
        aabb::{surrounding_box, AABB},
        hittable::{HitRecord, Hittable},
    },
    ray::Ray,
};

#[derive(Default)]
pub struct BVHNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    hitbox: AABB,
}

impl BVHNode {
    pub fn new(src_list: Vec<Box<dyn Hittable>>, time_frame: (f64, f64)) -> Self {
        let mut new = BVHNode {
            left: None,
            right: None,
            hitbox: AABB::default(),
        };

        let mut list = src_list;

        if list.len() == 1 {
            new.left = list.pop();
            new.hitbox = new.left.as_ref().unwrap().bounding_box(time_frame).unwrap();
        } else if list.len() == 2 {
            let mut rng = thread_rng();
            list.shuffle(&mut rng);
            new.left = list.pop();
            new.right = list.pop();
            new.hitbox = surrounding_box(
                new.left.as_ref().unwrap().bounding_box(time_frame).unwrap(),
                new.right
                    .as_ref()
                    .unwrap()
                    .bounding_box(time_frame)
                    .unwrap(),
            );
        } else {
            let mut rng = thread_rng();
            let axis: u8 = rng.gen_range(0..3);
            list.sort_by(|a, b| match axis {
                0 => a
                    .bounding_box((0.0, 0.0))
                    .unwrap()
                    .min()
                    .x()
                    .total_cmp(&b.bounding_box((0.0, 0.0)).unwrap().min().x()),
                1 => a
                    .bounding_box((0.0, 0.0))
                    .unwrap()
                    .min()
                    .y()
                    .total_cmp(&b.bounding_box((0.0, 0.0)).unwrap().min().y()),
                _ => a
                    .bounding_box((0.0, 0.0))
                    .unwrap()
                    .min()
                    .z()
                    .total_cmp(&b.bounding_box((0.0, 0.0)).unwrap().min().z()),
            });

            let middle = list.len() / 2;
            let mut left_list: Vec<Box<dyn Hittable>> = vec![];
            let mut right_list: Vec<Box<dyn Hittable>> = vec![];

            for _ in 0..middle {
                left_list.push(list.pop().unwrap());
            }
            for _ in 0..list.len() {
                right_list.push(list.pop().unwrap());
            }

            let left = BVHNode::new(left_list, time_frame);
            let right = BVHNode::new(right_list, time_frame);

            new.hitbox = surrounding_box(left.hitbox, right.hitbox);

            new.left = Some(Box::new(left));
            new.right = Some(Box::new(right));
        }

        new
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        if !self.hitbox.hit(r, interval) {
            return None;
        }

        let mut hitrecord = None;

        if let Some(box_left) = &self.left {
            if let Some(hit_left) = box_left.hit(r, interval) {
                hitrecord = Some(hit_left);
            }
        }

        let t_max = match &hitrecord {
            Some(record) => record.t,
            None => interval.1,
        };

        if let Some(box_right) = &self.right {
            if let Some(hit_right) = box_right.hit(r, (interval.0, t_max)) {
                hitrecord = Some(hit_right);
            }
        }

        hitrecord
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        Some(self.hitbox)
    }
}
