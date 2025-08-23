use std::{io::stdin, sync::Arc};

use ray_tracing::{
    figures::{
        bvh::BvhNode,
        camera::{Camera, DefocusSettings, ImageSettings, ViewSettings},
        constant_medium::ConstantMedium,
        cube::Cube,
        hittable_list::HitList,
        quad::{QDisk, QRect, QTri},
        rotate::Rotate,
        sphere::Sphere,
        translate::Translate,
    },
    materials::{
        dielectric::Dielectric, emissive::DiffuseLight, lambertian::Lambertian, material::Material,
        metal::Metal,
    },
    textures::{checker::CheckerTexture, image::ImageTexture, noise::NoiseTexture},
    utility::{
        color::Color,
        utils::random_f32,
        vec3::{Point3, Precision, Vec3},
    },
};

fn match_scene(scene: i32) {
    match scene {
        0 => eprintln!("INVALID ARGUMENT! Only input an integer to select a scene"),
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => weekend_final(),
        6 => quads(),
        7 => cube(),
        8 => simple_light(),
        9 => cornell_box(),
        10 => cornell_smoke(),
        11 => next_week_final(800, 10000, 40),
        -1 => next_week_final(400, 250, 4), // debug
        12 => monte_carlo(),
        n => eprintln!("{n} is not a valid scene..."),
    }
}

fn main() {
    let mut args = std::env::args();

    // program name
    args.next();

    if let Some(arg) = args.next() {
        match_scene(arg.parse().unwrap_or_default());
    } else {
        eprintln!("Scenes:");
        eprintln!("1: Bouncing spheres");
        eprintln!("2: Checkered spheres");
        eprintln!("3: Earth");
        eprintln!("4: Perlin spheres");
        eprintln!("5: Final render of the weekend");
        eprintln!("6: Quads scene");
        eprintln!("7: Single cube");
        eprintln!("8: Simple light");
        eprintln!("9: Cornell box");
        eprintln!("10: Cornell smoke");
        eprintln!("11: Next week final render");
        eprintln!("12: Monte carlo");

        eprintln!("Choose a scene: ");
        let stdin = stdin();
        let mut input = String::with_capacity(5);
        assert!(stdin.read_line(&mut input).is_ok());
        match_scene(input.trim_end().parse().unwrap_or_default());
    }
}

fn monte_carlo() {
    let mut inside_circle = 0_i64;
    let mut inside_circle_stratified = 0_i64;
    const N: i32 = 1000000;
    let sqrt_n = (N as f32).sqrt().round() as i32;

    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let x = random_f32(-1., 1.);
            let y = random_f32(-1., 1.);
            if x*x + y*y < 1. { inside_circle += 1 }

            let x = 2. * ((i as f32 + fastrand::f32()) / sqrt_n as f32) - 1.;
            let y = 2. * ((j as f32 + fastrand::f32()) / sqrt_n as f32) - 1.;
            if x*x + y*y < 1. { inside_circle_stratified += 1 }
        }
    }

    println!("Regular estimate of PI = {:.12}", (4. * inside_circle as f64) / N as f64);
    println!("Stratified estimate of PI = {:.12}", (4. * inside_circle_stratified as f64) / N as f64);
}

fn next_week_final(image_width: i32, samples_per_pixel: i32, max_depth: i32) {
    let mut boxes1 = HitList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as Precision * w;
            let z0 = -1000. + j as Precision * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = random_f32(1., 101.);
            let z1 = z0 + w;

            boxes1.push(Arc::new(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HitList::new();

    world.push(Arc::new(BvhNode::from_hitlist(boxes1)));

    let light = Arc::new(DiffuseLight::from_color(Color::new(1., 1., 1.), 7.));
    world.push(Arc::new(QRect::new(
        Point3::new(123., 554., 147.),
        Vec3::new(300., 0., 0.),
        Vec3::new(0., 0., 265.),
        light.clone(),
    )));

    let center1 = Point3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.push(Arc::new(Sphere::new_animated(
        center1,
        center2,
        50.,
        sphere_material,
    )));

    let glass = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        glass.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        glass.clone(),
    ));
    world.push(boundary.clone());
    world.push(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(Point3::new(0., 0., 0.), 5000., glass.clone()));
    world.push(Arc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::new(1., 1., 1.),
    )));

    let emat = Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.push(Arc::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.push(Arc::new(Sphere::new(
        Point3::new(220., 280., 300.),
        80.,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let mut boxes2 = HitList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.push(Arc::new(Sphere::new(
            Point3::random_bounded(0., 165.),
            10.,
            white.clone(),
        )));
    }

    world.push(Arc::new(Translate::new(
        Arc::new(Rotate::new(
            Arc::new(BvhNode::from_hitlist(boxes2)),
            Vec3::new(0., 15., 0.),
        )),
        Vec3::new(-100., 270., 395.),
    )));

    let image_settings = ImageSettings {
        aspect_ratio: 1.,
        image_width,
        samples_per_pixel,
        max_depth,
        background: Color::new(0., 0., 0.),
    };
    let view_settings = ViewSettings {
        vfov: 40.,
        look_from: Point3::new(478., 278., -600.),
        look_at: Point3::new(278., 278., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(image_settings, view_settings, Default::default());

    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HitList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(1., 1., 1.), 7.));

    world.push(Arc::new(QRect::new(
        Point3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        green.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(0., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        red.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(113., 554., 127.),
        Vec3::new(330., 0., 0.),
        Vec3::new(0., 0., 305.),
        light.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        white.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(555., 555., 555.),
        Vec3::new(-555., 0., 0.),
        Vec3::new(0., 0., -555.),
        white.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(0., 0., 555.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        white.clone(),
    )));

    let box1 = Arc::new(Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    ));
    let box1 = Arc::new(Rotate::new(box1, Vec3::new(0., 15., 0.)));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));

    let box2 = Arc::new(Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    ));
    let box2 = Arc::new(Rotate::new(box2, Vec3::new(0., -18., 0.)));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));

    world.push(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(0., 0., 0.),
    )));
    world.push(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new(1., 1., 1.),
    )));

    let image_settings = ImageSettings {
        aspect_ratio: 1.,
        image_width: 600,
        samples_per_pixel: 200,
        max_depth: 50,
        background: Color::new(0., 0., 0.),
    };
    let view_settings = ViewSettings {
        vfov: 40.,
        look_from: Point3::new(278., 278., -800.),
        look_at: Point3::new(278., 278., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(image_settings, view_settings, DefocusSettings::default());

    cam.render(&world);
}

fn cornell_box() {
    let mut world = HitList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(1., 1., 1.), 15.));

    world.push(Arc::new(QRect::new(
        Point3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        green.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(0., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        red.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(343., 554., 332.),
        Vec3::new(-130., 0., 0.),
        Vec3::new(0., 0., -105.),
        light.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        white.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(555., 555., 555.),
        Vec3::new(-555., 0., 0.),
        Vec3::new(0., 0., -555.),
        white.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(0., 0., 555.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        white.clone(),
    )));

    let box1 = Arc::new(Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    ));
    let box1 = Arc::new(Rotate::new(box1, Vec3::new(0., 15., 0.)));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
    world.push(box1);

    let box2 = Arc::new(Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    ));
    let box2 = Arc::new(Rotate::new(box2, Vec3::new(0., -18., 0.)));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    world.push(box2);

    let image_settings = ImageSettings {
        aspect_ratio: 1.,
        image_width: 600,
        samples_per_pixel: 200,
        max_depth: 50,
        background: Color::new(0., 0., 0.),
    };
    let view_settings = ViewSettings {
        vfov: 40.,
        look_from: Point3::new(278., 278., -800.),
        look_at: Point3::new(278., 278., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(image_settings, view_settings, DefocusSettings::default());

    cam.render(&world);
}

fn simple_light() {
    let mut world = HitList::new();

    let pertext = Arc::new(NoiseTexture::new(4.));
    let mat = Arc::new(Lambertian::from_texture(pertext.clone()));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        mat.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        mat.clone(),
    )));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(1., 0.2, 0.), 4.));
    world.push(Arc::new(QRect::new(
        Point3::new(3., 1., -2.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 2., 0.),
        difflight.clone(),
    )));

    let image_settings = ImageSettings {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0., 0., 0.),
    };
    let view_settings = ViewSettings {
        vfov: 20.,
        look_from: Point3::new(26., 3., 6.),
        look_at: Point3::new(0., 2., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(image_settings, view_settings, DefocusSettings::default());

    cam.render(&world);
}

fn cube() {
    let mut world = HitList::new();

    let red = Arc::new(Lambertian::new(Color::new(1., 0.2, 0.2)));

    world.push(Arc::new(Cube::new(
        Point3::new(-1., -1., -1.),
        Point3::new(1., 1., 1.),
        red,
    )));

    let image_settings = ImageSettings {
        aspect_ratio: 1.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.),
    };
    let view_settings = ViewSettings {
        vfov: 80.,
        look_from: Point3::new(5., 5., 5.),
        look_at: Point3::new(0., 0., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(image_settings, view_settings, DefocusSettings::default());

    cam.render(&world);
}

fn quads() {
    let mut world = HitList::new();

    // Materials
    let left_red = Arc::new(Lambertian::new(Color::new(1., 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1., 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1., 0.5, 0.)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.push(Arc::new(QDisk::new(
        Point3::new(-3., 0., 3.),
        Vec3::new(0., 0., -2.),
        Vec3::new(0., 2., 0.),
        left_red.clone(),
    )));
    world.push(Arc::new(QTri::new(
        Point3::new(-2., -2., 0.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 4., 0.),
        back_green.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(3., -2., 1.),
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 4., 0.),
        right_blue.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(-2., 3., 1.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., 4.),
        upper_orange.clone(),
    )));
    world.push(Arc::new(QRect::new(
        Point3::new(-2., -3., 5.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., -4.),
        lower_teal.clone(),
    )));

    let image_settings = ImageSettings {
        aspect_ratio: 1.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.),
    };
    let view_settings = ViewSettings {
        vfov: 80.,
        look_from: Point3::new(0., 0., 9.),
        look_at: Point3::new(0., 0., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(image_settings, view_settings, DefocusSettings::default());

    cam.render(&world);
}

fn weekend_final() {
    // Camera

    let image_settings = ImageSettings {
        aspect_ratio: 16. / 9.,
        image_width: 1200,
        samples_per_pixel: 500,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.),
    };
    let view_settings = ViewSettings {
        vfov: 20.,
        look_from: Point3::new(13., 2., 3.),
        look_at: Point3::new(0., 0., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let defocus_settings = DefocusSettings {
        defocus_angle: 0.6,
        focus_dist: 10.,
    };
    let camera = Camera::new(image_settings, view_settings, defocus_settings);

    // World

    let mut world = HitList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material.clone(),
    )));

    // glass.
    let material_1 = Arc::new(Dielectric::new(1.5));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f32();
            let center = Point3::new(
                a as Precision + 0.9 * fastrand::f32(),
                0.2,
                b as Precision + 0.9 * fastrand::f32(),
            );

            if (center - Point3::new(4., 0.2, 0.)).len() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse.

                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal.

                    let albedo = Color::random_bounded(0.5, 1.);
                    let fuzz = random_f32(0., 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass.

                    material_1.clone()
                };

                world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    world.push(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material_1.clone(),
    )));

    let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material_2.clone(),
    )));

    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.push(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material_3.clone(),
    )));

    camera.render(&world);
}

fn perlin_spheres() {
    let mut world = HitList::new();

    let pertext = Arc::new(NoiseTexture::new(4.));
    let material = Arc::new(Lambertian::from_texture(pertext));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        material.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        material.clone(),
    )));

    let view_settings = ViewSettings {
        vfov: 20.,
        look_from: Point3::new(13., 2., 3.),
        look_at: Point3::new(0., 0., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(
        ImageSettings::default(),
        view_settings,
        DefocusSettings::default(),
    );

    cam.render(&world);
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface));

    let view_settings = ViewSettings {
        vfov: 20.,
        look_from: Point3::new(0., 0., 12.),
        look_at: Point3::new(0., 0., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(
        ImageSettings::default(),
        view_settings,
        DefocusSettings::default(),
    );

    cam.render(&globe);
}

fn checkered_spheres() {
    let mut world = HitList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.push(Arc::new(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Arc::new(Lambertian::from_texture(checker.clone())),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Arc::new(Lambertian::from_texture(checker)),
    )));

    let view_settings = ViewSettings {
        vfov: 20.,
        look_from: Point3::new(13., 2., 3.),
        look_at: Point3::new(0., 0., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(
        ImageSettings::default(),
        view_settings,
        DefocusSettings::default(),
    );

    cam.render(&world);
}

fn bouncing_spheres() {
    // Camera

    let image_settings = ImageSettings {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.),
    };
    let view_settings = ViewSettings {
        vfov: 20.,
        look_from: Point3::new(13., 2., 3.),
        look_at: Point3::new(0., 0., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let defocus_settings = DefocusSettings {
        defocus_angle: 0.6,
        focus_dist: 10.,
    };
    let camera = Camera::new(image_settings, view_settings, defocus_settings);

    // World

    let mut world = HitList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::from_texture(checker));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material.clone(),
    )));

    // glass.
    let material_1 = Arc::new(Dielectric::new(1.5));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f32();
            let center = Point3::new(
                a as Precision + 0.9 * fastrand::f32(),
                0.2,
                b as Precision + 0.9 * fastrand::f32(),
            );

            if (center - Point3::new(4., 0.2, 0.)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse.

                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0., random_f32(0., 0.5), 0.);
                    world.push(Arc::new(Sphere::new_animated(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal.

                    let albedo = Color::random_bounded(0.5, 1.);
                    let fuzz = random_f32(0., 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass.

                    let sphere_material = material_1.clone();
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    world.push(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material_1.clone(),
    )));

    let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material_2.clone(),
    )));

    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.push(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material_3.clone(),
    )));

    let world = BvhNode::from_hitlist(world);

    camera.render(&world);
}
