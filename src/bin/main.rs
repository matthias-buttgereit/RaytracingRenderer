use std::{fs::File, io::BufWriter, path::Path, time::Instant};

use raytracing::{
    ray::Ray,
    vec3::{dot, unit_vector, Color, Point3, Vec3},
    write_color,
};

fn ray_color(r: Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, 1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = r.origin() - center;
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discrim = b * b - 4.0 * a * c;

    discrim > 0.0
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // PNG File
    let mut data: Vec<u8> = Vec::with_capacity((3 * image_width * image_height) as usize);

    let path = Path::new(r"images\test.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set_color(png::ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();

    // Render
    let start = Instant::now();
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(r);
            write_color(&mut data, pixel_color);
        }
    }
    writer.write_image_data(&data).unwrap();

    let end = start.elapsed();

    eprintln!(
        "\nFinished in {}:{} minutes!",
        end.as_secs() / 60,
        end.as_secs() % 60
    );
}
