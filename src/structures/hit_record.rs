use crate::structures::{Vec3, Point3, Ray};

#[derive(Clone, Copy, Default, Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f64) -> Self {
        Self {
            point: point,
            normal: normal.normalized(),
            t: t
        }
    }

    pub fn is_front_facing(&self, ray: &Ray) -> bool {
        Vec3::dot(&self.normal, &ray.direction) < 0.0
    }
}