use std::sync::Arc;

use crate::utility::{ray::Ray, utils::degrees_to_radians, vec3::{Point3, Precision, Vec3}};

use super::{aabb::AABB, hittable::Hittable};

pub struct Rotate {
    object: Arc<dyn Hittable>,
    rotation_matrix: [Vec3; 3],
    inverse_rotation: [Vec3; 3],
    bbox: AABB,
}

impl Rotate {
    pub fn new(object: Arc<dyn Hittable>, angles: Vec3) -> Self {
        let alpha = degrees_to_radians(-angles[0]);
        let beta = degrees_to_radians(-angles[1]);
        let gamma = degrees_to_radians(-angles[2]);
        let cosa = alpha.cos();
        let cosb = beta.cos();
        let cosg = gamma.cos();
        let sina = alpha.sin();
        let sinb = beta.sin();
        let sing = gamma.sin();

        let rotation_matrix = [
            Vec3::new(
                cosa * cosb,
                cosa * sinb * sing - sina * cosg,
                cosa * sinb * cosg + sina * sing,
            ),
            Vec3::new(
                sina * cosb,
                sina * sinb * sing + cosa * cosg,
                sina * sinb * sing - cosa * sing,
            ),
            Vec3::new(-sinb, cosb * sing, cosb * cosg),
        ];

        let inverse_rotation = [
            Vec3::new(
                cosa * cosb,
                cosa * sinb * sing + sina * cosg,
                cosa * -sinb * cosg + sina * sing,
            ),
            Vec3::new(
                -sina * cosb,
                -sina * sinb * sing + cosa * cosg,
                -sina * sinb * sing + cosa * sing,
            ),
            Vec3::new(sinb, cosb * -sing, cosb * cosg),
        ];

        let bbox = object.bounding_box();

        let mut min = Point3::new(Precision::INFINITY, Precision::INFINITY, Precision::INFINITY);
        let mut max = Point3::new(Precision::NEG_INFINITY, Precision::NEG_INFINITY, Precision::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as Precision;
                    let j = j as Precision;
                    let k = k as Precision;
                    let x = i*bbox.x().max + (1.-i)*bbox.x().min;
                    let y = j*bbox.y().max + (1.-j)*bbox.y().min;
                    let z = k*bbox.z().max + (1.-k)*bbox.z().min;

                    let tester = rotate_vec3(&rotation_matrix, Vec3::new(x, y, z));

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::from_points(min, max);

        Self {
            object,
            rotation_matrix,
            inverse_rotation,
            bbox,
        }
    } 
}

fn rotate_vec3(rotation_matrix: &[Vec3; 3], v: Vec3) -> Vec3 {
    let mut res = Vec3::default();
    for row in 0..3 {
        res[row] = rotation_matrix[row as usize].dot(&v);
    }

    res
}

impl Hittable for Rotate {
    fn hit(
        &self,
        r: &crate::utility::ray::Ray,
        ray_t: crate::utility::interval::Interval,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        // Transform the ray from world space to object space
        let rotated_ray = {
            let orig = rotate_vec3(&self.rotation_matrix, *r.origin());
            let dir = rotate_vec3(&self.rotation_matrix, *r.direction());
            &Ray::new(orig, dir)
        };

        // Determine whether an intersection exists in object space (and if so, where).
        if !self.object.hit(rotated_ray, ray_t, rec) {
            return false;
        }

        // Transform the intersection from object space back to world space.
        rec.p = rotate_vec3(&self.inverse_rotation, rec.p);
        rec.normal = rotate_vec3(&self.inverse_rotation, rec.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
