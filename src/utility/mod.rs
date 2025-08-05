
pub mod vec3;
pub mod color;
pub mod ray;
pub mod interval;

pub mod utils {
    // use std::f64::consts::PI as pi64;
    pub use std::f32::consts::PI as pi32;

    use super::vec3::Precision;

    #[inline(always)]
    pub fn degrees_to_radians(degrees: Precision) -> Precision {
        degrees * pi32 / 180.0
    }

    #[inline(always)]
    pub fn random_f32(min: f32, max: f32) -> f32 {
        min + (max - min) * fastrand::f32()
    }

    #[inline(always)]
    pub fn random_i32(min: i32, max: i32) -> i32 {
        fastrand::i32(min..=max)
    }
}
