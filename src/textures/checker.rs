use std::sync::Arc;

use crate::utility::{color::Color, vec3::Precision};

use super::texture::{SolidColor, Texture};

#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: Precision,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: Precision, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1. / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: Precision, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1. / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: Precision, v: Precision, p: crate::utility::vec3::Point3) -> Color {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
