use std::sync::Arc;

use crate::{
    figures::hittable::HitRecord,
    textures::texture::{SolidColor, Texture},
    utility::{color::Color, ray::Ray, vec3::Vec3},
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
        let mut scatter_direction = rec.normal + Vec3::random_unit_vec();

        // Catch degenerate scatter direction.
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        Some(ScatteredRay {
            ray: Ray::with_time(rec.p, scatter_direction, ray.time()),
            attenuation: self.tex.value(rec.u, rec.v, rec.p),
        })
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian::new(Color::default())
    }
}
