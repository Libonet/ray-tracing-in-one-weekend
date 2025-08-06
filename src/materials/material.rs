use crate::{
    figures::hittable::HitRecord,
    utility::{color::Color, ray::Ray},
};

use super::lambertian::Lambertian;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Color,
}

pub fn default_material() -> Lambertian {
    Lambertian::default()
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatteredRay>;
}
