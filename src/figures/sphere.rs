use std::rc::Rc;

use crate::{
    materials::material::Material,
    utility::{
        interval::Interval,
        ray::Ray,
        vec3::{Point3, Precision},
    },
};

use super::hittable::{HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: Precision,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: Precision, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius: Precision::max(0., radius),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().len_square();
        let h = r.direction().dot(&oc);
        let c = oc.len_square() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range.
        let root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            let root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.mat.clone();

        true
    }
}
