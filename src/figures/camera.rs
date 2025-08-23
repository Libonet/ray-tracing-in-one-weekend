use crate::{
    image_formats::ppm::PPM,
    utility::{
        color::Color,
        interval::Interval,
        ray::Ray,
        utils::degrees_to_radians,
        vec3::{Point3, Precision, Vec3},
    },
};

use super::hittable::{HitRecord, Hittable};

pub struct ImageSettings {
    pub aspect_ratio: Precision,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub background: Color,
}

impl Default for ImageSettings {
    fn default() -> Self {
        let aspect_ratio = 16. / 9.;
        let image_width = 400;
        let samples_per_pixel = 100;
        let max_depth = 50;
        let background = Color::new(0.7, 0.8, 1.);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            background,
        }
    }
}

pub struct ViewSettings {
    pub vfov: Precision,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
}

impl Default for ViewSettings {
    fn default() -> Self {
        let vfov = 90.;
        let look_from = Point3::new(0., 0., 0.);
        let look_at = Point3::new(0., 0., -1.);
        let vup = Vec3::new(0., 1., 0.);

        Self {
            vfov,
            look_from,
            look_at,
            vup,
        }
    }
}

pub struct DefocusSettings {
    pub defocus_angle: Precision,
    pub focus_dist: Precision,
}

impl Default for DefocusSettings {
    fn default() -> Self {
        let defocus_angle = 0.;
        let focus_dist = 10.;

        Self {
            defocus_angle,
            focus_dist,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Camera {
    // Image settings.
    aspect_ratio: Precision,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    background: Color,

    // View settings.
    vfov: Precision,
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,

    // Calculated from inputs.
    pixel_samples_scale: Precision, // color scale factor for a sum of pixel samples
    image_height: i32,              // height of rendered image
    sqrt_spp: i32,                  // square root of number of samples per pixel
    recip_sqrt_spp: Precision,      // 1 / sqrt_spp
    center: Point3,                 // camera center
    pixel00_loc: Point3,            // location of pixel 0,0
    pixel_delta_u: Vec3,            // offset to pixel to the right
    pixel_delta_v: Vec3,            // offset to pixel below

    // Camera frame basis vectors.
    u: Vec3,
    v: Vec3,
    w: Vec3,

    // Defocus settings.
    defocus_angle: Precision,
    focus_dist: Precision,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        image_settings: ImageSettings,
        view_settings: ViewSettings,
        defocus_settings: DefocusSettings,
    ) -> Self {
        // Calculate the image height, and ensure that it's at least 1.

        let image_height =
            (image_settings.image_width as Precision / image_settings.aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let sqrt_spp = (image_settings.samples_per_pixel as Precision).sqrt() as i32;
        let pixel_samples_scale = 1.0 / image_settings.samples_per_pixel as Precision;
        let recip_sqrt_spp = 1. / sqrt_spp as Precision;

        // Camera

        let center = view_settings.look_from;

        let theta = degrees_to_radians(view_settings.vfov);
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h * defocus_settings.focus_dist;
        let viewport_width =
            viewport_height * (image_settings.image_width as Precision / image_height as Precision);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (view_settings.look_from - view_settings.look_at).unit_vec();
        let u = view_settings.vup.cross(&w).unit_vec();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.

        let pixel_delta_u = viewport_u / image_settings.image_width as Precision;
        let pixel_delta_v = viewport_v / image_height as Precision;

        // Calculate the location of the upper left pixel.

        let viewport_upper_left =
            center - (defocus_settings.focus_dist * w) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.

        let defocus_radius = defocus_settings.focus_dist
            * degrees_to_radians(defocus_settings.defocus_angle / 2.).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio: image_settings.aspect_ratio,
            image_width: image_settings.image_width,
            samples_per_pixel: image_settings.samples_per_pixel,
            max_depth: image_settings.max_depth,
            background: image_settings.background,
            vfov: view_settings.vfov,
            look_from: view_settings.look_from,
            look_at: view_settings.look_at,
            vup: view_settings.vup,

            pixel_samples_scale,
            image_height,
            sqrt_spp,
            recip_sqrt_spp,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,

            u,
            v,
            w,

            defocus_angle: defocus_settings.defocus_angle,
            focus_dist: defocus_settings.focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        let ppm = PPM::generate(
            self.image_width as usize,
            self.image_height as usize,
            255,
            |row, col| {
                let mut pixel_color = Color::new(0., 0., 0.);

                for s_j in 0..self.sqrt_spp {
                    for s_i in 0..self.sqrt_spp {
                        let r = self.get_ray(col as i32, row as i32, s_i, s_j);
                        pixel_color += self.ray_color(&r, self.max_depth, world);
                    }
                }

                self.pixel_samples_scale * pixel_color
            },
        );

        ppm.output();
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        let mut rec = HitRecord::default();

        if !world.hit(r, Interval::new(0.001, Precision::INFINITY), &mut rec) {
            return self.background;
        }

        let color_from_emission = rec.material.emitted(rec.u, rec.v, rec.p);

        let scattered_ray = rec.material.scatter(r, &rec);
        if scattered_ray.is_none() {
            return color_from_emission;
        }

        let scaterred_ray = scattered_ray.unwrap();
        let color_from_scatter = scaterred_ray.attenuation * self.ray_color(&scaterred_ray.ray, depth-1, world);

        color_from_emission + color_from_scatter
    }

    /// Construct a camera ray originating from the defocus disk and directed at randomly
    /// sampled point around the pixel location i, j for stratified sample square s_i, s_j.
    fn get_ray(&self, i: i32, j: i32, s_i: i32, s_j: i32) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_center = self.pixel00_loc
            + ((i as Precision + offset.x()) * self.pixel_delta_u)
            + ((j as Precision + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_center - ray_origin;
        let ray_time = fastrand::f32();

        Ray::with_time(ray_origin, ray_direction, ray_time)
    }

    fn sample_square_stratified(&self, s_i: i32, s_j: i32) -> Vec3 {
        let px = ((s_i as f32 + fastrand::f32()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f32 + fastrand::f32()) * self.recip_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0.)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(fastrand::f32() - 0.5, fastrand::f32() - 0.5, 0.)
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let image_settings = ImageSettings::default();
        let view_settings = ViewSettings::default();
        let defocus_settings = DefocusSettings::default();

        Self::new(image_settings, view_settings, defocus_settings)
    }
}

#[inline(always)]
pub fn lerp(start: Color, end: Color, progress: Precision) -> Color {
    (1.0 - progress) * start + progress * end
}
