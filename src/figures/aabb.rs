use crate::utility::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x() <= b.x() { Interval::new(a.x(), b.x()) } else { Interval::new(b.x(), a.x()) };
        let y = if a.y() <= b.y() { Interval::new(a.y(), b.y()) } else { Interval::new(b.y(), a.y()) };
        let z = if a.z() <= b.z() { Interval::new(a.z(), b.z()) } else { Interval::new(b.z(), a.z()) };

        Self { x, y, z }
    }

    pub fn concat(&mut self, rhs: AABB) {
        self.x.concat(rhs.x);
        self.y.concat(rhs.y);
        self.z.concat(rhs.z);
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min { ray_t.min = t0 }
                if t1 < ray_t.max { ray_t.max = t1 }
            } else {
                if t1 > ray_t.min { ray_t.min = t1 }
                if t0 < ray_t.max { ray_t.max = t0 }
            }

            if ray_t.max <= ray_t.min { return false }
        }

        true
    }
}

