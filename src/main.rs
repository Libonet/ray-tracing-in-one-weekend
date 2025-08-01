use std::rc::Rc;

use ray_tracing::{
    figures::{camera::{Camera, DefocusSettings, ImageSettings, ViewSettings}, sphere::Sphere},
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    utility::{color::Color, vec3::{Point3, Vec3}},
};

fn main() {
    // Camera

    let image_settings = ImageSettings::default();
    let view_settings = ViewSettings {
        vfov: 20.,
        look_from: Point3::new(-2., 2., 1.),
        look_at: Point3::new(0., 0., -1.),
        vup: Vec3::new(0., 1., 0.),
    };
    let defocus_settings = DefocusSettings {
        defocus_angle: 10.,
        focus_dist: 3.4,
    };
    let camera = Camera::new(image_settings, view_settings, defocus_settings);

    // World

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1. / 1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    let world = vec![
        Rc::new(Sphere::new(
            Point3::new(0., -100.5, -1.),
            100.,
            material_ground.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(0., 0., -1.2),
            0.5,
            material_center.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(-1., 0., -1.),
            0.5,
            material_left.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(-1., 0., -1.),
            0.4,
            material_bubble.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(1., 0., -1.),
            0.5,
            material_right.clone(),
        )),
    ];

    camera.render(&world);
}
