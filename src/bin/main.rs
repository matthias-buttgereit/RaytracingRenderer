use std::{fs::File, io::BufWriter, path::Path, rc::Rc, time::Instant};

use png::{ColorType, Encoder};
use raytracing::{
    bvh_tree::bvh_node::BVHNode,
    camera::Camera,
    hits::{
        constant_medium::ConstantMedium, hittable::Hittable, rotate::RotateY, translate::Translate,
    },
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
        Material,
    },
    objects::{
        aa_rect::{XYRect, XZRect, YZRect},
        block::Block,
        moving_sphere::MovingSphere,
        sphere::Sphere,
    },
    random_f64, random_f64_between, ray_color,
    textures::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
    },
    vec3::{random_vector, random_vector_in_range, Color, Point3, Vec3},
    write_color,
};

#[allow(dead_code)]
fn random_scene(aspect_ratio: f64) -> (BVHNode, Camera, Color) {
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
                f64::from(a) + 0.15 + 0.85 * random_f64(),
                0.2,
                f64::from(b) + 0.15 + 0.85 * random_f64(),
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
            aspect_ratio,
            0.1,
            10.0,
            (0.0, 1.0),
        ),
        Color::new(0.7, 0.8, 1.0),
    )
}

#[allow(dead_code)]
fn two_spheres(aspect_ratio: f64) -> (BVHNode, Camera, Color) {
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
            aspect_ratio,
            0.0,
            20.0,
            (0.0, 1.0),
        ),
        Color::new(0.7, 0.8, 1.0),
    )
}

#[allow(dead_code)]
fn two_perlin_spheres(aspect_ratio: f64) -> (BVHNode, Camera, Color) {
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
            aspect_ratio,
            0.0,
            20.0,
            (0.0, 1.0),
        ),
        Color::new(0.7, 0.8, 1.0),
    )
}

#[allow(dead_code)]
fn earth(aspect_ratio: f64) -> (BVHNode, Camera, Color) {
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
            aspect_ratio,
            0.0,
            20.0,
            (0.0, 1.0),
        ),
        Color::new(0.7, 0.8, 1.0),
    )
}

#[allow(dead_code)]
fn simple_light(aspect_ratio: f64) -> (BVHNode, Camera, Color) {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let noise_texture = Rc::new(NoiseTexture::new(4.0));
    let metal = Rc::new(Metal::new(Color::new(0.9, 0.9, 0.9), 0.1));

    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        metal,
    )));

    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new_from_texture(noise_texture)),
    )));

    let difflight = Rc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    objects.push(Box::new(XYRect::new(
        (3.0, 5.0),
        (1.0, 3.0),
        -2.0,
        difflight,
    )));

    let redlight = Rc::new(DiffuseLight::new(Color::new(10.0, 2.0, 2.0)));
    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        redlight,
    )));

    (
        BVHNode::new(objects, (0.0, 1.0)),
        Camera::new(
            Point3::new(26.0, 3.0, 6.0),
            Point3::new(0.0, 2.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.0,
            20.0,
            (0.0, 1.0),
        ),
        Color::default(),
    )
}

#[allow(dead_code)]
fn cornell_box(aspect_ratio: f64) -> (BVHNode, Camera, Color) {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    objects.push(Box::new(YZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        green,
    )));
    objects.push(Box::new(YZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red)));
    objects.push(Box::new(XZRect::new(
        (213.0, 343.0),
        (227.0, 332.0),
        554.0,
        light,
    )));
    objects.push(Box::new(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        white.clone(),
    )));
    objects.push(Box::new(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    )));
    objects.push(Box::new(XYRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    )));

    let block1 = Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let block1 = RotateY::new(Box::new(block1), -18.0);
    let block1 = Translate::new(Box::new(block1), Vec3::new(130.0, 0.0, 65.0));
    objects.push(Box::new(block1));

    let block2 = Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white,
    );
    let block2 = RotateY::new(Box::new(block2), 15.0);
    let block2 = Translate::new(Box::new(block2), Vec3::new(265.0, 0.0, 295.0));
    objects.push(Box::new(block2));

    (
        BVHNode::new(objects, (0.0, 1.0)),
        Camera::new(
            Point3::new(278.0, 278.0, -800.0),
            Point3::new(278.0, 278.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            0.0,
            10.0,
            (0.0, 1.0),
        ),
        Color::default(),
    )
}

#[allow(dead_code)]
fn cornell_smoke(aspect_ratio: f64) -> (BVHNode, Camera, Color) {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    objects.push(Box::new(YZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        green,
    )));
    objects.push(Box::new(YZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red)));
    objects.push(Box::new(XZRect::new(
        (113.0, 443.0),
        (127.0, 432.0),
        554.0,
        light,
    )));
    objects.push(Box::new(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        white.clone(),
    )));
    objects.push(Box::new(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    )));
    objects.push(Box::new(XYRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    )));

    let block1 = Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let block1 = RotateY::new(Box::new(block1), -18.0);
    let block1 = Translate::new(Box::new(block1), Vec3::new(130.0, 0.0, 65.0));
    objects.push(Box::new(ConstantMedium::new_from_color(
        Box::new(block1),
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    let block2 = Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white,
    );
    let block2 = RotateY::new(Box::new(block2), 15.0);
    let block2 = Translate::new(Box::new(block2), Vec3::new(265.0, 0.0, 295.0));
    objects.push(Box::new(ConstantMedium::new_from_color(
        Box::new(block2),
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    (
        BVHNode::new(objects, (0.0, 1.0)),
        Camera::new(
            Point3::new(278.0, 278.0, -800.0),
            Point3::new(278.0, 278.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            0.0,
            20.0,
            (0.0, 1.0),
        ),
        Color::default(),
    )
}

#[allow(dead_code)]
fn final_scene(aspect_ratio: f64) -> (BVHNode, Camera, Color) {
    // Ground
    let mut boxes1: Vec<Box<dyn Hittable>> = vec![];
    let ground = Rc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + f64::from(i) * w;
            let z0 = -1000.0 + f64::from(j) * w;
            let y0 = 0.0;

            let x1 = x0 + w;
            let y1 = random_f64_between(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(Box::new(Block::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects: Vec<Box<dyn Hittable>> = vec![Box::new(BVHNode::new(boxes1, (0.0, 1.0)))];

    // Light
    let light = Rc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    objects.push(Box::new(XZRect::new(
        (123.0, 423.0),
        (147.0, 412.0),
        554.0,
        light,
    )));

    // Moving Sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objects.push(Box::new(MovingSphere::new(
        (center1, center2),
        (0.0, 1.0),
        50.0,
        moving_sphere_material,
    )));

    // Metal and Glas Sphere
    objects.push(Box::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // ConstantMediums
    let boundary = Box::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    objects.push(boundary);
    let boundary = Box::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    objects.push(Box::new(ConstantMedium::new_from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    objects.push(Box::new(ConstantMedium::new_from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    // Earth
    let earth_mat = Rc::new(Lambertian::new_from_texture(Rc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    objects.push(Box::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_mat,
    )));

    // Perlin Sphere
    let pertext = Rc::new(NoiseTexture::new(0.1));
    objects.push(Box::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::new_from_texture(pertext)),
    )));

    // Translation + Rotation
    let mut boxes2: Vec<Box<dyn Hittable>> = vec![];
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1_000;
    for _ in 0..ns {
        boxes2.push(Box::new(Sphere::new(
            random_vector_in_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.push(Box::new(Translate::new(
        Box::new(RotateY::new(
            Box::new(BVHNode::new(boxes2, (0.0, 1.0))),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    (
        BVHNode::new(objects, (0.0, 1.0)),
        Camera::new(
            Vec3::new(478.0, 278.0, -600.0),
            Vec3::new(278.0, 278.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            0.0,
            20.0,
            (0.0, 1.0),
        ),
        Color::default(),
    )
}

fn main() {
    // Image
    let aspect_ratio = 1.0;
    let image_width: u32 = 400;
    let image_height: u32 = (f64::from(image_width) / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let max_depth = 50;

    // World + Camera
    let (world, camera, background) = cornell_box(aspect_ratio);

    // PNG File
    let mut data: Vec<u8> = Vec::with_capacity((3 * image_width * image_height) as usize);

    let path = Path::new(r"images\test.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = Encoder::new(w, image_width, image_height);
    encoder.set_color(ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();

    // Render
    let start = Instant::now();

    for j in (0..image_height).rev() {
        eprint!(
            "\r{} / {} lines rendered...",
            image_height - j,
            image_height
        );
        //eprint!("\rProgress: {}%", ((image_height - j) * 100) / image_height);
        for i in 0..image_width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + random_f64()) / f64::from(image_width - 1);
                let v = (f64::from(j) + random_f64()) / f64::from(image_height - 1);
                let r = camera.get_ray(u, v);

                pixel_color += ray_color(r, &background, &world, max_depth);
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
        "\nFinished in {}:{:02} minutes!",
        end.as_secs() / 60,
        end.as_secs() % 60
    );
}
