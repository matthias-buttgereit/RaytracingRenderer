use std::{fs::File, io::BufWriter, path::Path, rc::Rc, time::Instant};

use raytracing::{
    bvh_tree::bvh_node::BVHNode,
    camera::Camera,
    hits::hittable::Hittable,
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    objects::{moving_sphere::MovingSphere, sphere::Sphere},
    random_f64, random_f64_between, ray_color,
    textures::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
    },
    vec3::{random_vector, random_vector_in_range, Color, Point3, Vec3},
    write_color,
};

#[allow(dead_code)]
fn random_scene() -> (BVHNode, Camera) {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let checker = Rc::new(CheckerTexture::new_from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let ground_material = Lambertian::new_from_texture(checker);
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(ground_material),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_f64();
            let center = Point3::new(
                a as f64 + 0.15 + 0.85 * random_f64(),
                0.2,
                b as f64 + 0.15 + 0.85 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_material < 0.8 {
                    //diffuse
                    let albedo = random_vector() * random_vector();
                    let sphere_material = Lambertian::new(albedo);
                    let center2 = center + Vec3::new(0.0, random_f64_between(0.0, 0.5), 0.0);
                    world.push(Box::new(MovingSphere::new(
                        (center, center2),
                        (0.0, 1.0),
                        0.2,
                        Rc::new(sphere_material),
                    )));
                } else if choose_material < 0.95 {
                    //metal
                    let albedo: Color = random_vector_in_range(0.5, 1.0);
                    let fuzz = random_f64_between(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.push(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                } else {
                    //glass
                    let sphere_material = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(material1),
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(material2),
    )));

    let material3 = Metal::new(Color::new(0.7, 0.8, 0.5), 0.0);
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(material3),
    )));

    (
        BVHNode::new(world, (0.0, 1.0)),
        Camera::new(
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            16.0 / 9.0,
            0.1,
            10.0,
            (0.0, 1.0),
        ),
    )
}

#[allow(dead_code)]
fn two_spheres() -> (BVHNode, Camera) {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let checker = Rc::new(CheckerTexture::new_from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let checker_material: Rc<dyn Material> = Rc::new(Lambertian::new_from_texture(checker));

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker_material.clone(),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker_material.clone(),
    )));

    (
        BVHNode::new(world, (0.0, 1.0)),
        Camera::new(
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            20.0,
            16.0 / 9.0,
            0.0,
            20.0,
            (0.0, 1.0),
        ),
    )
}

#[allow(dead_code)]
fn two_perlin_spheres() -> (BVHNode, Camera) {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let pertext = Rc::new(NoiseTexture::new(4.0));
    let pertext_material: Rc<dyn Material> = Rc::new(Lambertian::new_from_texture(pertext));

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        pertext_material.clone(),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        pertext_material.clone(),
    )));

    (
        BVHNode::new(world, (0.0, 1.0)),
        Camera::new(
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            20.0,
            16.0 / 9.0,
            0.0,
            20.0,
            (0.0, 1.0),
        ),
    )
}

#[allow(dead_code)]
fn earth() -> (BVHNode, Camera) {
    let mut globe: Vec<Box<dyn Hittable>> = vec![];

    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface: Rc<dyn Material> = Rc::new(Lambertian::new_from_texture(earth_texture));

    globe.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));

    (
        BVHNode::new(globe, (0.0, 1.0)),
        Camera::new(
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            20.0,
            16.0 / 9.0,
            0.0,
            20.0,
            (0.0, 1.0),
        ),
    )
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 800;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 50;
    let max_depth = 20;

    // World + Camera
    let (world, camera) = earth();

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
        eprint!("\rProgress: {}%", ((image_height - j) * 100) / image_height);
        for i in 0..image_width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);

                pixel_color += ray_color(r, &world, max_depth);
                if j == 0 && i == 0 {
                    pixel_color = Color::new(1.0, 0.0, 0.0);
                }
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
