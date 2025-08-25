use crate::utility::utils::PI;

use crate::utility::{orthonormal::Onb, vec3::{Precision, Vec3}};

pub trait Pdf {
    fn value(&self, direction: Vec3) -> Precision;
    fn generate(&self) -> Vec3;
}

pub struct SpherePdf;

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3) -> Precision {
        1. / (4. * PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vec()
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(normal: Vec3) -> Self {
        Self { uvw: Onb::new(normal) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> Precision {
        let cosine_theta = direction.unit_vec().dot(&self.uvw.w());
        cosine_theta.max(0.) / PI
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(Vec3::random_cosine_direction())
    }
}
