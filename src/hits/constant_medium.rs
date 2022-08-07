use std::rc::Rc;

use crate::{
    materials::{isotropic::Isotropic, Material},
    random_f64,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
};

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    phase_function: Rc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Box<dyn Hittable>, d: f64, a: Rc<dyn Material>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: (-1.0) / d,
            phase_function: a,
        }
    }

    pub fn new_from_color(b: Box<dyn Hittable>, d: f64, c: Color) -> Self {
        Self {
            boundary: b,
            phase_function: Rc::new(Isotropic::new_from_color(c)),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self, time: (f64, f64)) -> Option<AABB> {
        self.boundary.bounding_box(time)
    }

    fn hit(&self, r: &Ray, interval: (f64, f64)) -> Option<HitRecord> {
        let enable_debug = false;
        let debugging = enable_debug && random_f64() < 0.00001;
        let (t_min, t_max) = interval;

        match self.boundary.hit(r, (-f64::INFINITY, f64::INFINITY)) {
            None => None,
            Some(mut rec1) => match self.boundary.hit(r, (rec1.t + 0.0001, f64::INFINITY)) {
                None => None,
                Some(mut rec2) => {
                    if debugging {
                        eprintln!("\nt_min={}, t_max={}", rec1.t, rec2.t);
                    }

                    if rec1.t < t_min {
                        rec1.t = t_min;
                    }
                    if rec2.t > t_max {
                        rec2.t = t_max;
                    }

                    if rec1.t >= rec2.t {
                        return None;
                    }

                    if rec1.t < 0.0 {
                        rec1.t = 0.0;
                    }

                    let ray_length = r.direction().len();
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                    let hit_distance = self.neg_inv_density * random_f64().ln();

                    if hit_distance > distance_inside_boundary {
                        return None;
                    }

                    let rec_t = rec1.t + hit_distance / ray_length;
                    let rec_p = r.at(rec_t);

                    if debugging {
                        eprintln!(
                            "hit_distance = {}\nrec.t = {}\nrec.p = {:?}",
                            hit_distance, rec_t, rec_p
                        );
                    }

                    let rec_normal = Vec3::new(1.0, 0.0, 0.0);
                    let rec_front_face = true;
                    let rec_mat_ptr = self.phase_function.clone();

                    Some(HitRecord {
                        t: rec_t,
                        p: rec_p,
                        normal: rec_normal,
                        front_face: rec_front_face,
                        material: rec_mat_ptr,
                        surface_coordinates: (0.0, 0.0),
                    })
                }
            },
        }
    }
}
