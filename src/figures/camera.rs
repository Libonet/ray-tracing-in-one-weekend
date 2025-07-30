use crate::{image_formats::ppm::PPM, utility::{color::Color, interval::Interval, ray::Ray, vec3::{Point3, Precision, Vec3}}};

use super::hittable::{HitRecord, Hittable};



pub struct Camera {
    aspect_ratio: Precision,
    image_width: i32,
    image_height: i32,   // height of rendered image
    center: Point3,      // camera center
    pixel00_loc: Point3, // location of pixel 0,0
    pixel_delta_u: Vec3, // offset to pixel to the right
    pixel_delta_v: Vec3, // offset to pixel below
}

impl Camera {
    pub fn new(aspect_ratio: Precision, image_width: i32) -> Self {
        
        // Calculate the image height, and ensure that it's at least 1.

        let image_height = (image_width as Precision / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        // Camera

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (image_width as Precision / image_height as Precision);
        let center = Point3::new(0., 0., 0.);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.

        let pixel_delta_u = viewport_u / image_width as Precision;
        let pixel_delta_v = viewport_v / image_height as Precision;

        // Calculate the location of the upper left pixel.

        let viewport_upper_left = 
            center - Vec3::new(0., 0., focal_length)
                          - viewport_u/2.
                          - viewport_v/2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self { 
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        let ppm = PPM::generate(self.image_width as usize, self.image_height as usize, 255, 
            |row,col| {
                let pixel_center = self.pixel00_loc + (col * self.pixel_delta_u) + (row * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                Camera::ray_color(&r, world)
        });

        ppm.output();
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0., Precision::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction().unit_vec();
        let a = 0.5 * (unit_direction.y() + 1.0);
        lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), a)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16. / 9.;
        let image_width = 400;

        Self::new(aspect_ratio, image_width)
    }
}

#[inline(always)]
pub fn lerp(start: Color, end: Color, progress: Precision) -> Color {
    (1.0 - progress) * start + progress * end
}
