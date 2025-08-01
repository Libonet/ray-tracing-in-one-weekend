use std::rc::Rc;

use ray_tracing::{
    figures::{
        camera::{Camera, DefocusSettings, ImageSettings, ViewSettings},
        sphere::Sphere,
    },
    materials::{dielectric::Dielectric, lambertian::Lambertian, material::Material, metal::Metal},
    utility::{
        color::Color,
        utils::random_f32,
        vec3::{Point3, Precision, Vec3},
    },
};

fn main() {
    // Camera

    let image_settings = ImageSettings {
        aspect_ratio: 16. / 9.,
        image_width: 1200,
        samples_per_pixel: 500,
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

    let mut world = Vec::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Rc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material.clone(),
    )));

    // glass.
    let material_1 = Rc::new(Dielectric::new(1.5));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f32();
            let center = Point3::new(
                a as Precision + 0.9 * fastrand::f32(),
                0.2,
                b as Precision + 0.9 * fastrand::f32(),
            );

            if (center - Point3::new(4., 0.2, 0.)).len() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse.

                    let albedo = Color::random() * Color::random();
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal.

                    let albedo = Color::random_bounded(0.5, 1.);
                    let fuzz = random_f32(0., 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass.

                    material_1.clone()
                };

                world.push(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    world.push(Rc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material_1.clone(),
    )));

    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Rc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material_2.clone(),
    )));

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.push(Rc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material_3.clone(),
    )));

    camera.render(&world);
}
