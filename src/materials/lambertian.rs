use std::{f32::consts::PI, sync::Arc};

use crate::{
    figures::hittable::HitRecord,
    textures::texture::{SolidColor, Texture},
    utility::{color::Color, orthonormal::Onb, ray::Ray, vec3::Vec3},
};

use super::material::{Material, ScatteredRay};

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let uvw = Onb::new(rec.normal);
        let scatter_direction = uvw.transform(Vec3::random_cosine_direction());
        // let scatter_direction = Vec3::random_on_hemisphere(&rec.normal);

        let scattered = Ray::with_time(rec.p, scatter_direction.unit_vec(), ray.time());
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        // let attenuation = 0.5 * (rec.normal + Vec3::new(1., 1., 1.));
        let pdf = uvw.w().dot(scattered.direction()) / PI;

        Some(ScatteredRay { ray: scattered, attenuation, pdf })
    }

    fn scattering_pdf(&self, _ray: &Ray, rec: &HitRecord, scattered: &Ray) -> crate::utility::vec3::Precision {
        let cos_theta = rec.normal.dot(&scattered.direction().unit_vec());
        cos_theta.max(0.) / PI
    }

    // fn scattering_pdf(&self, _ray: &Ray, _rec: &HitRecord, _scattered: &Ray) -> crate::utility::vec3::Precision {
    //     1. / (2. * PI)
    // }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian::new(Color::default())
    }
}
