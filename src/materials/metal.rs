use crate::{
    figures::hittable::HitRecord,
    utility::{
        color::Color,
        interval::Interval,
        ray::Ray,
        vec3::{Precision, Vec3},
    },
};

use super::material::{Material, ScatteredRay};

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: Precision,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Precision) -> Self {
        let fuzz = if Interval::new(0., 1.).contains(fuzz) {
            fuzz
        } else {
            1.
        };

        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let reflected = ray.direction().reflect(rec.normal);
        let reflected = reflected.unit_vec() + (self.fuzz * Vec3::random_unit_vec());
        let scattered = Ray::with_time(rec.p, reflected, ray.time());
        let attenuation = self.albedo;

        (scattered.direction().dot(&rec.normal) > 0.).then_some(ScatteredRay {
            ray: scattered,
            attenuation,
        })
    }
}
