use vec3::Color;

pub mod vec3;

pub fn write_color(list: &mut Vec<u8>, color: Color) {
    list.push(color.x() as u8);
    list.push(color.y() as u8);
    list.push(color.z() as u8);
}
