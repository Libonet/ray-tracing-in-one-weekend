use crate::utility::{interval::Interval, ray::Ray};

use super::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct HitList<T> {
    list: Vec<T>,
    bbox: AABB,
}

impl<T: Hittable> HitList<T> {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            bbox: AABB::default(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.bbox.concat(value.bounding_box());
        self.list.push(value);
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.bbox = AABB::default();
    }
}

impl<T: Hittable> Hittable for HitList<T> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for item in self.list.iter() {
            if item.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
