use crate::{
    figures::hittable::HitRecord,
    utility::{color::Color, ray::Ray, vec3::{Point3, Precision}},
};

use super::lambertian::Lambertian;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Color,
    pub pdf: Precision,
}

pub fn default_material() -> Lambertian {
    Lambertian::default()
}

pub trait Material: Send + Sync {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<ScatteredRay> {
        None
    }

    fn emitted(&self, _ray: &Ray, _rec: &HitRecord, _u: Precision, _v: Precision, _p: Point3) -> Color {
        Color::new(0., 0., 0.)
    }

    fn scattering_pdf(&self, _ray: &Ray, _rec: &HitRecord, _scattered: &Ray) -> Precision {
        1.
    }
}
