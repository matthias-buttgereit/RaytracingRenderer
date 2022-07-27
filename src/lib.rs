use hits::hittable::Hittable;
use ray::Ray;
use vec3::{dot, unit_vector, Color};

pub mod camera;
pub mod hits;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod vec3;

pub fn write_color(list: &mut Vec<u8>, color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (color.x() * scale).sqrt();
    let g = (color.y() * scale).sqrt();
    let b = (color.z() * scale).sqrt();

    list.push((256.0 * clamp(r, (0.0, 0.999))) as u8);
    list.push((256.0 * clamp(g, (0.0, 0.999))) as u8);
    list.push((256.0 * clamp(b, (0.0, 0.999))) as u8);
}

fn clamp(x: f64, range: (f64, f64)) -> f64 {
    let (min, max) = range;
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn ray_color(r: Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::default();
    }

    if let Some(hitrecord) = world.hit(&r, (0.001, f64::INFINITY)) {
        let (scattered, attenuation) = hitrecord.material.scatter(r, &hitrecord);
        if dot(&scattered.direction(), &hitrecord.normal) > 0.0 {
            return attenuation * ray_color(scattered, world, depth - 1);
        } else {
            return Color::default();
        }
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
