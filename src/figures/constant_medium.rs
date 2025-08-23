use std::sync::Arc;

use crate::{materials::{isotropic::Isotropic, material::Material}, textures::texture::Texture, utility::{color::Color, interval::Interval, vec3::{Precision, Vec3}}};

use super::hittable::{HitRecord, Hittable};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: Precision,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: Precision, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: Precision, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &crate::utility::ray::Ray, ray_t: crate::utility::interval::Interval, rec: &mut super::hittable::HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, Interval::new(rec1.t+0.0001, Precision::INFINITY), &mut rec2) {
            return false;
        }

        rec1.t = rec1.t.max(ray_t.min);
        rec2.t = rec2.t.min(ray_t.max);

        if rec1.t >= rec2.t { return false; }

        if rec1.t < 0. { rec1.t = 0.; }

        let ray_length = r.direction().len();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * fastrand::f32().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1., 0., 0.); // Arbitrary
        rec.front_face = true; // Arbitrary too
        rec.material = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> super::aabb::AABB {
        self.boundary.bounding_box()
    }
}
