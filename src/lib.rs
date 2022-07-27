use vec3::Color;

pub mod camera;
pub mod hits;
pub mod objects;
pub mod ray;
pub mod vec3;

pub fn write_color(list: &mut Vec<u8>, color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = color.x() * scale;
    let g = color.y() * scale;
    let b = color.z() * scale;

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
