use ray_tracing::utility::{color::Color, ray::Ray, vec3::{Point3, Precision, Vec3}};
use ray_tracing::image_formats::ppm::PPM;

fn main() {
    // Image

    let aspect_ratio = 16. / 9.;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.

    let image_height = (image_width as Precision / aspect_ratio) as usize;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    
    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (image_width as Precision / image_height as Precision);
    let camera_center = Point3::new(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.

    let pixel_delta_u = viewport_u / image_width as Precision;
    let pixel_delta_v = viewport_v / image_height as Precision;

    // Calculate the location of the upper left pixel.

    let viewport_upper_left = 
        camera_center - Vec3::new(0., 0., focal_length)
                      - viewport_u/2.
                      - viewport_v/2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render.

    let ppm = PPM::generate(image_width, image_height, 255, 
        |row,col| {
            let pixel_center = pixel00_loc + (col * pixel_delta_u) + (row * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            ray_color(&r)
    });

    ppm.output();

    eprintln!("\nDone! :D");
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let normal = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vec();
        return 0.5 * Color::new(normal.x()+1.0, normal.y()+1.0, normal.z()+1.0);
    }

    let unit_direction = r.direction().unit_vec();
    let a = 0.5 * (unit_direction.y() + 1.0);
    lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), a)
}

#[inline(always)]
fn lerp(start: Color, end: Color, progress: Precision) -> Color {
    (1.0 - progress) * start + progress * end
}

fn hit_sphere(center: &Point3, radius: Precision, r: &Ray) -> Precision {
    let oc = *center - *r.origin();
    let a = r.direction().len_square();
    let h = r.direction().dot(&oc);
    let c = oc.len_square() - radius * radius;
    let discriminant = h*h - a*c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}




