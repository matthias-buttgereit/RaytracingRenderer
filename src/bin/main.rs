use std::{fs::File, io::BufWriter, path::Path, rc::Rc, time::Instant};

use rand::{thread_rng, Rng};
use raytracing::{
    camera::Camera,
    hits::hittalbe_list::HittableList,
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    objects::sphere::Sphere,
    random_f64, ray_color,
    vec3::{random_vector, random_vector_in_range, Color, Point3, Vec3},
    write_color,
};

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(ground_material),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_f64();
            let center = Point3::new(
                a as f64 + 0.8 * random_f64(),
                0.2,
                b as f64 + 0.8 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_material < 0.8 {
                    //diffuse
                    let albedo = random_vector() * random_vector();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                } else if choose_material < 0.95 {
                    //metal
                    let albedo: Color = random_vector_in_range(0.5, 1.0);
                    let fuzz = thread_rng().gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                } else {
                    //glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(material1),
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(material2),
    )));

    let material3 = Metal::new(Color::new(0.7, 0.8, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(material3),
    )));
    world
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

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
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + thread_rng().gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + thread_rng().gen_range(0.0..1.0)) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);

                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(&mut data, pixel_color, samples_per_pixel);
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
