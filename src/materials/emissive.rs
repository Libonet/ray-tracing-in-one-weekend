use std::sync::Arc;

use crate::{textures::texture::{SolidColor, Texture}, utility::{color::Color, vec3::{Point3, Precision}}};

use super::material::Material;

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(emit: Color, intensity: Precision) -> Self {
        let light = emit.unit_vec() / (emit.x().max(emit.y()).max(emit.z())) * intensity;
        Self { tex: Arc::new(SolidColor::new(light)) }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &crate::utility::ray::Ray, _rec: &crate::figures::hittable::HitRecord) -> Option<super::material::ScatteredRay> {
        None
    }

    fn emitted(&self, u: Precision, v: Precision, p: Point3) -> Color {
        self.tex.value(u, v, p)
    }
}
