use crate::structures::{Vec3, Point3, Ray};
use crate::materials::Material;

use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, material: Arc<dyn Material>, t: f64, u: f64, v: f64) -> Self {
        Self {
            point: point,
            normal: normal.normalized(),
            material: material,
            t: t,
            u: u,
            v: v
        }
    }

    pub fn is_front_facing(&self, ray: &Ray) -> bool {
        Vec3::dot(&self.normal, &ray.direction) < 0.0
    }
}