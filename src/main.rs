use std::rc::Rc;

use ray_tracing::{figures::{camera::Camera, sphere::Sphere}, utility::vec3::Point3};

fn main() {

    // Camera

    let camera = Camera::default();

    // World
    
    let world = vec![
        Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    camera.render(&world);
}



