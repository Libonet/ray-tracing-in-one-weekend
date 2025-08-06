use std::{cmp::Ordering, sync::Arc};

use crate::utility::{interval::Interval, ray::Ray};

use super::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HitList,
};

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn from_hitlist(list: HitList<Arc<dyn Hittable>>) -> Self {
        Self::new(list.objects())
    }

    pub fn new(mut objects: Vec<Arc<dyn Hittable>>) -> Self {
        assert!(!objects.is_empty());
        let mut bbox = AABB::EMPTY;
        for object in objects.iter() {
            bbox.concat(object.bounding_box());
        }

        let axis = bbox.longest_axis();

        let (left, right) = match objects.len() {
            1 => {
                let obj = objects.remove(0);

                (obj.clone(), obj)
            }
            2 => {
                let right = objects.remove(1);
                let left = objects.remove(0);

                (left, right)
            }
            _ => {
                let mut obj_clone = objects.to_vec();
                obj_clone.sort_by(|a, b| BvhNode::box_compare(a, b, axis));

                let mid = objects.len() / 2;
                let left_vec = objects[..mid].to_vec();
                let right_vec = objects[mid..].to_vec();

                let left = Arc::new(BvhNode::new(left_vec));
                let right = Arc::new(BvhNode::new(right_vec));

                (left as Arc<dyn Hittable>, right as Arc<dyn Hittable>)
            }
        };

        Self { left, right, bbox }
    }

    pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);

        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let end = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, end), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
