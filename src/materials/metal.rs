use crate::{figures::hittable::HitRecord, utility::{color::Color, ray::Ray}};

use super::material::{Material, ScatteredRay};

#[derive(Debug)]
pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let reflected = ray.direction().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        Some(ScatteredRay { ray: scattered, attenuation })
    }
}
