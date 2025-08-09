use std::sync::Arc;

use crate::{
    materials::material::Material,
    utility::vec3::{Point3, Vec3},
};

use super::{hittable::Hittable, hittable_list::HitList, quad::QRect};

pub struct Cube {
    faces: HitList<Arc<dyn Hittable>>,
}

impl Cube {
    pub fn new(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Self {
        let mut faces: HitList<Arc<dyn Hittable>> = HitList::new();

        // Get the min and max coordinates for the points a and b
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0., 0.);
        let dy = Vec3::new(0., max.y() - min.y(), 0.);
        let dz = Vec3::new(0., 0., max.z() - min.z());

        // backface
        faces.push(Arc::new(QRect::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dy,
            mat.clone(),
        )));

        // frontface
        faces.push(Arc::new(QRect::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            mat.clone(),
        )));

        // leftface
        faces.push(Arc::new(QRect::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            mat.clone(),
        )));

        // rightface
        faces.push(Arc::new(QRect::new(
            Point3::new(max.x(), min.y(), min.z()),
            dz,
            dy,
            mat.clone(),
        )));

        // upface
        faces.push(Arc::new(QRect::new(
            Point3::new(min.x(), max.y(), min.z()),
            dx,
            dz,
            mat.clone(),
        )));

        // downface
        faces.push(Arc::new(QRect::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            mat,
        )));

        Self { faces }
    }
}

impl Hittable for Cube {
    fn hit(
        &self,
        r: &crate::utility::ray::Ray,
        ray_t: crate::utility::interval::Interval,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        self.faces.hit(r, ray_t, rec)
    }

    fn bounding_box(&self) -> super::aabb::AABB {
        self.faces.bounding_box()
    }
}
