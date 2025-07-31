use crate::{figures::hittable::HitRecord, utility::{color::Color, ray::Ray, vec3::Vec3}};

use super::material::{Material, ScatteredRay};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vec();

        // Catch degenerate scatter direction.
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        Some(ScatteredRay {
            ray: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
