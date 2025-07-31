use std::rc::Rc;

use ray_tracing::{
    figures::{camera::Camera, sphere::Sphere},
    materials::{lambertian::Lambertian, metal::Metal},
    utility::{color::Color, vec3::Point3},
};

fn main() {
    // Camera

    let camera = Camera::default();

    // World

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let world = vec![
        Rc::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.2),
            0.5,
            material_center.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right.clone(),
        )),
    ];

    camera.render(&world);
}
