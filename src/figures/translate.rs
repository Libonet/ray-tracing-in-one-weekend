use std::sync::Arc;

use crate::utility::{ray::Ray, vec3::Vec3};

use super::{aabb::AABB, hittable::Hittable};



pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;

        Self { object, offset, bbox }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: crate::utility::interval::Interval, rec: &mut super::hittable::HitRecord) -> bool {
        let offset_ray = Ray::with_time(*r.origin() - self.offset, *r.direction(), r.time());

        if !self.object.hit(&offset_ray, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
