use std::{marker::PhantomData, sync::Arc};

use crate::{
    materials::material::Material,
    utility::{
        interval::Interval,
        vec3::{Point3, Precision, Vec3},
    },
};

use super::{aabb::AABB, hittable::Hittable};

pub trait QuadPrimitive: Send + Sync {
    fn is_interior(alpha: Precision, beta: Precision, rec: &mut super::hittable::HitRecord)
        -> bool;

    fn set_bounding_box(q: Point3, u: Vec3, v: Vec3) -> AABB {
        let mut bbox_diagonal1 = AABB::from_points(q, q + u + v);
        let bbox_diagonal2 = AABB::from_points(q + u, q + v);
        bbox_diagonal1.concat(bbox_diagonal2);

        bbox_diagonal1
    }
}

#[derive(Clone)]
pub struct Quad<T: QuadPrimitive> {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: Precision,
    w: Vec3,

    phantom: PhantomData<T>,
}

impl<T: QuadPrimitive> Quad<T> {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox = T::set_bounding_box(q, u, v);

        let n = u.cross(&v);
        let normal = n.unit_vec();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);

        Self {
            q,
            u,
            v,
            mat,
            bbox,
            normal,
            d,
            w,
            phantom: PhantomData,
        }
    }
}

impl<T: QuadPrimitive> Hittable for Quad<T> {
    fn hit(
        &self,
        r: &crate::utility::ray::Ray,
        ray_t: crate::utility::interval::Interval,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        let denom = self.normal.dot(r.direction());

        // No hit if ray is parallel
        if denom.abs() < 1e-4 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if !T::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.material = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

pub struct Rect;
pub type QRect = Quad<Rect>;

impl QuadPrimitive for Rect {
    fn is_interior(
        alpha: Precision,
        beta: Precision,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        let unit_interval = Interval::new(0., 1.);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }

        rec.u = alpha;
        rec.v = beta;

        true
    }
}

pub struct Tri;
pub type QTri = Quad<Tri>;

impl QuadPrimitive for Tri {
    fn is_interior(
        alpha: Precision,
        beta: Precision,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        if alpha <= 0. || beta <= 0. || alpha + beta >= 1. {
            return false;
        }

        rec.u = alpha;
        rec.v = beta;

        true
    }
}

pub struct Disk;
pub type QDisk = Quad<Disk>;

impl QuadPrimitive for Disk {
    fn is_interior(
        alpha: Precision,
        beta: Precision,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        if alpha.powi(2) + beta.powi(2) > 1. {
            return false;
        }

        rec.u = alpha / 2. + 0.5;
        rec.v = beta / 2. + 0.5;

        true
    }

    fn set_bounding_box(q: Point3, u: Vec3, v: Vec3) -> AABB {
        AABB::from_points(q - u - v, q + u + v)
    }
}
