use crate::structures::{AABB, HitRecord, Point3, Ray, Vec3};
use crate::hittables::Hittable;
use crate::materials::Material;
use crate::utility::InverseLerp;

use std::sync::Arc;

pub struct XZRect {
    pub material: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub y: f64
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, y: f64, material: Arc<dyn Material>) -> Self {
        Self {
            x0: x0,
            x1: x1,
            z0: z0,
            z1: z1,
            y: y,
            material: material
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.y - ray.origin.y) / ray.direction.y;
        
        if t < t_min || t > t_max {
            return None
        }

        let p = ray.at(t);

        if (p.x < self.x0 || p.x > self.x1) || (p.z < self.z0 || p.z > self.z1) {
            return None
        }

        let u = f64::inverse_lerp(self.x0, self.x1, p.x);
        let v = f64::inverse_lerp(self.z0, self.z1, p.z);

        Some(HitRecord::new(p, Vec3::up(), self.material.clone(), t, u, v))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        let aabb = AABB::new(Point3::new(self.x0, self.y - 0.0001, self.z0), Point3::new(self.x1, self.y + 0.0001, self.z1));
        Some(aabb)
    }
}