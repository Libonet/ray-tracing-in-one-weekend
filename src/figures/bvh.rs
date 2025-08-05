use std::{cmp::Ordering, rc::Rc};

use crate::utility::{interval::Interval, ray::Ray, utils::random_i32};

use super::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
};

#[derive(Clone)]
pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    // pub fn from_hitlist(list: HitList<Rc<dyn Hittable>>) -> Self {
    //     Self::new(list.objects())
    // }

    pub fn new(mut objects: Vec<Rc<dyn Hittable>>) -> Self {
        assert!(!objects.is_empty());
        let axis = random_i32(0, 2);

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

                let left = Rc::new(BvhNode::new(left_vec));
                let right = Rc::new(BvhNode::new(right_vec));

                (left as Rc<dyn Hittable>, right as Rc<dyn Hittable>)
            }
        };

        let mut bbox = left.bounding_box();
        bbox.concat(right.bounding_box());

        Self { left, right, bbox }
    }

    pub fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: i32) -> Ordering {
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
