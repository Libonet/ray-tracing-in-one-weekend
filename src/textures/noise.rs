use crate::utility::{color::Color, perlin::PerlinNoise, vec3::Precision};

use super::texture::Texture;

#[derive(Clone, Debug, Default)]
pub struct NoiseTexture {
    noise: PerlinNoise,
    scale: Precision,
}

impl NoiseTexture {
    pub fn new(scale: Precision) -> Self {
        Self {
            scale,
            noise: PerlinNoise::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(
        &self,
        _u: crate::utility::vec3::Precision,
        _v: crate::utility::vec3::Precision,
        p: crate::utility::vec3::Point3,
    ) -> crate::utility::color::Color {
        Color::new(0.5, 0.5, 0.5)
            * (1. + (self.scale * p.z() + 10. * self.noise.turbulence(p, 7)).sin())
    }
}
