use std::{ops::Deref, rc::Rc};

use crate::{
    materials::material::{self, Material},
    utility::{
        interval::Interval,
        ray::Ray,
        vec3::{Point3, Precision, Vec3},
    },
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: Precision,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        // assert_eq!(outward_normal.len(), 1.);

        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Default::default(),
            normal: Default::default(),
            material: Rc::new(material::default_material()),
            t: Default::default(),
            front_face: Default::default(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for item in self.iter() {
            if item.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

impl<T: Hittable> Hittable for Rc<T> {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        Rc::deref(self).hit(r, ray_t, rec)
    }
}
