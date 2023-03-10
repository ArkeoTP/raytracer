use crate::primitives::vector::{Point3, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
