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
        let emit = if emit.near_zero() { Color::new(1., 1., 1.) } else { emit };
        let light = intensity * (3. / emit.len_square()) * emit;
        Self { tex: Arc::new(SolidColor::new(light)) }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &crate::utility::ray::Ray, _rec: &crate::figures::hittable::HitRecord) -> Option<super::material::ScatteredRay> {
        None
    }

    fn emitted(&self, _ray: &crate::utility::ray::Ray, rec: &crate::figures::hittable::HitRecord, u: Precision, v: Precision, p: Point3) -> Color {
        if !rec.front_face {
            return Color::new(0., 0., 0.);
        }
        self.tex.value(u, v, p)
    }
}
