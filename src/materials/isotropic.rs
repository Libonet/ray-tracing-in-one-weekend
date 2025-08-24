use std::{f32::consts::PI, sync::Arc};

use crate::{
    figures::hittable::HitRecord,
    textures::texture::{SolidColor, Texture},
    utility::{color::Color, ray::Ray, vec3::Vec3},
};

use super::material::{Material, ScatteredRay};

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let ray = Ray::with_time(rec.p, Vec3::random_unit_vec(), ray.time());
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        let pdf = 1. / (4. * PI);

        Some(ScatteredRay { ray, attenuation, pdf })
    }

    fn scattering_pdf(&self, _ray: &Ray, _rec: &HitRecord, _scattered: &Ray) -> crate::utility::vec3::Precision {
        1. / (4. * PI)
    }
}
