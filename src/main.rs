use std::rc::Rc;

use ray_tracing::{
    figures::{camera::Camera, sphere::Sphere},
    materials::lambertian::Lambertian,
    utility::vec3::Point3,
};

fn main() {
    // Camera

    let camera = Camera::default();

    // World

    let lambert = Rc::new(Lambertian::default());
    let world = vec![
        Rc::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            lambert.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            lambert.clone(),
        )),
    ];

    camera.render(&world);
}
