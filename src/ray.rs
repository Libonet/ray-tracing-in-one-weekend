use crate::vec3::{Point3, Precision, Vec3};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn origin_mut(&mut self) -> &mut Point3 {
        &mut self.orig
    }

    pub fn direction_mut(&mut self) -> &mut Vec3 {
        &mut self.dir
    }

    pub fn at(&self, t: Precision) -> Point3 {
        self.orig + t*self.dir
    }
}
