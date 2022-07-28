use crate::{
    degrees_to_radians,
    ray::Ray,
    vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3}, random_f64_between,
};

#[derive(Default)]
#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    uvw: (Vec3, Vec3, Vec3),
    lens_radius: f64,
    time_frame: (f64, f64),
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time_frame: (f64, f64),
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            uvw: (u, v, w),
            lens_radius,
            time_frame,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let (u, v, _) = self.uvw;
        let (time0, time1) = self.time_frame;
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = u * rd.x() + v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random_f64_between(time0, time1),
        )
    }
}
