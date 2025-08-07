use std::{io::stdin, sync::Arc};

use ray_tracing::{
    figures::{
        bvh::BvhNode,
        camera::{Camera, DefocusSettings, ImageSettings, ViewSettings},
        hittable_list::HitList,
        sphere::Sphere,
    },
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    textures::{checker::CheckerTexture, image::ImageTexture},
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

        eprintln!("Choose a scene: ");
        let stdin = stdin();
        let mut input = String::with_capacity(5);
        assert!(stdin.read_line(&mut input).is_ok());
        match_scene(input.trim_end().parse().unwrap_or_default());
    }
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.,0.,0.), 2., earth_surface));

    let view_settings = ViewSettings {
        vfov: 20.,
        look_from: Point3::new(0., 0., 12.),
        look_at: Point3::new(0., 0., 0.),
        vup: Vec3::new(0., 1., 0.),
    };
    let cam = Camera::new(ImageSettings::default(), view_settings, DefocusSettings::default());

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
    let cam = Camera::new(ImageSettings::default(), view_settings, DefocusSettings::default());

    cam.render(&world);
}

fn bouncing_spheres() {
    // Camera

    let image_settings = ImageSettings {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
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
