use vec3::Color;

pub mod ray;
pub mod vec3;

pub fn write_color(list: &mut Vec<u8>, color: Color) {
    list.push((color.x() * 255.999) as u8);
    list.push((color.y() * 255.999) as u8);
    list.push((color.z() * 255.999) as u8);
}
