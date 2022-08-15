use std::path::Path;

use load_image::export::rgb::RGB;
use load_image::ImageData::RGB8;

use crate::{
    clamp,
    vec3::{self, Color},
};

use super::Texture;

pub struct ImageTexture {
    data: Vec<RGB<u8>>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(image_path: &str) -> Self {
        let path = Path::new(image_path);
        let image = load_image::load_path(path).unwrap();

        let data = if let RGB8(vector) = image.bitmap {
            vector
        } else {
            panic!()
        };

        Self {
            data,
            width: image.width,
            height: image.height,
        }
    }
}

impl Texture for ImageTexture {
    #[allow(unused_variables)]
    fn value(&self, uv: (f64, f64), p: &vec3::Point3) -> Color {
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = clamp(uv.0, (0.0, 1.0));
        let v = 1.0 - clamp(uv.1, (0.0, 1.0));

        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;

        if i >= self.width {
            i = self.width - 1;
        };
        if j >= self.height {
            j = self.height - 1;
        };

        let color = self.data[(j * self.width + i)];

        let scale = 1.0 / 255.0;

        Color::new(
            f64::from(color.r) * scale,
            f64::from(color.g) * scale,
            f64::from(color.b) * scale,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use load_image::ImageData::RGB8;

    #[test]
    fn load_image() {
        let path = Path::new("earthmap.jpg");
        let image = load_image::load_path(path).unwrap();

        let mut data = if let RGB8(vector) = image.bitmap {
            vector
        } else {
            panic!()
        };
        println!("{:?}", data.pop().unwrap());
    }
}
