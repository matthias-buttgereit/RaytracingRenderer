use std::{fs::File, io::BufWriter, path::Path, time::Instant};

use raytracing::{vec3::Color, write_color};

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    let mut data: Vec<u8> = Vec::with_capacity((3 * image_width * image_height) as usize);

    let path = Path::new(r"images\test.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set_color(png::ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();

    let start = Instant::now();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let r = 255.999 * r;
            let g = 255.999 * g;
            let b = 255.999 * b;

            let pixel_color = Color::new(r, g, b);

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
