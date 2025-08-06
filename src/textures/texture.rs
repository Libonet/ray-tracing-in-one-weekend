use crate::utility::{
    color::Color,
    vec3::{Point3, Precision},
};

pub trait Texture {
    fn value(&self, u: Precision, v: Precision, p: Point3) -> Color;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn new_rgb(red: Precision, green: Precision, blue: Precision) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: Precision, _v: Precision, _p: Point3) -> Color {
        self.albedo
    }
}
