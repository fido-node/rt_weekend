use crate::vectors::{Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray { origin: origin.clone(), direction: direction.clone() }
    }
}
