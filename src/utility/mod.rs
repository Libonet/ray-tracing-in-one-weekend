pub mod color;
pub mod interval;
pub mod perlin;
pub mod ray;
pub mod vec3;
pub mod orthonormal;

pub mod utils {
    pub use std::f64::consts::PI;

    use super::vec3::Precision;

    #[inline(always)]
    pub fn degrees_to_radians(degrees: Precision) -> Precision {
        degrees * PI / 180.0
    }

    #[inline(always)]
    pub fn random_f32(min: f32, max: f32) -> f32 {
        min + (max - min) * fastrand::f32()
    }

    #[inline(always)]
    pub fn random_f64(min: f64, max: f64) -> f64 {
        min + (max - min) * fastrand::f64()
    }

    #[inline(always)]
    pub fn random_i32(min: i32, max: i32) -> i32 {
        fastrand::i32(min..=max)
    }

    #[inline(always)]
    pub fn random_i64(min: i64, max: i64) -> i64 {
        fastrand::i64(min..=max)
    }
}
