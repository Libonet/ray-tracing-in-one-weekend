use crate::{figures::hittable::HitRecord, utility::{color::Color, ray::Ray, vec3::Precision}};

use super::material::{Material, ScatteredRay};



pub struct Dielectric {
    refraction_index: Precision,
}

impl Dielectric {
    pub fn new(refraction_index: Precision) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let attenuation = Color::new(1., 1., 1.);
        let ri = if rec.front_face { 1. / self.refraction_index } else { self.refraction_index };

        let unit_direction = ray.direction().unit_vec();
        let cos_theta = (-unit_direction).dot(&rec.normal).clamp(-1., 1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refract = ri * sin_theta > 1.;

        let direction = if cannot_refract {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };

        let scattered = Ray::with_time(rec.p, direction, ray.time());

        Some(ScatteredRay { ray: scattered, attenuation })
    }
}
