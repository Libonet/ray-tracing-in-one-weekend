use std::rc::Rc;

use crate::{
    materials::material::Material,
    utility::{
        interval::Interval,
        ray::Ray,
        vec3::{Point3, Precision, Vec3},
    },
};

use super::hittable::{HitRecord, Hittable};

pub struct Sphere {
    center: Ray,
    radius: Precision,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: Precision, mat: Rc<dyn Material>) -> Self {
        Self {
            center: Ray::new(static_center, Vec3::new(0., 0., 0.)),
            radius: Precision::max(0., radius),
            mat,
        }
    }

    pub fn new_animated(center1: Point3, center2: Point3, radius: Precision, mat: Rc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1),
            radius: Precision::max(0., radius),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
        let a = r.direction().len_square();
        let h = r.direction().dot(&oc);
        let c = oc.len_square() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.mat.clone();

        true
    }
}
