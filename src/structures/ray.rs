use crate::structures::{Vec3, Point3};

#[derive(Clone, Copy, Default, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin: origin,
            direction: direction.normalized(),
            time: 0.0
        }
    }

    pub fn with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin: origin,
            direction: direction.normalized(),
            time: time
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}