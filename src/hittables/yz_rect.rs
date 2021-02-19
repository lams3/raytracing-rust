use crate::structures::{AABB, HitRecord, Point3, Ray, Vec3};
use crate::hittables::Hittable;
use crate::materials::Material;
use crate::utility::InverseLerp;

use std::sync::Arc;

pub struct YZRect {
    pub material: Arc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub x: f64
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, x: f64, material: Arc<dyn Material>) -> Self {
        Self {
            y0: y0,
            y1: y1,
            z0: z0,
            z1: z1,
            x: x,
            material: material
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.x - ray.origin.x) / ray.direction.x;
        
        if t < t_min || t > t_max {
            return None
        }

        let p = ray.at(t);

        if (p.y < self.y0 || p.y > self.y1) || (p.z < self.z0 || p.z > self.z1) {
            return None
        }

        let u = f64::inverse_lerp(self.y0, self.y1, p.x);
        let v = f64::inverse_lerp(self.z0, self.z1, p.z);

        Some(HitRecord::new(p, Vec3::right(), self.material.clone(), t, u, v))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        let aabb = AABB::new(Point3::new(self.x - 0.0001, self.y0, self.z0), Point3::new(self.x + 0.0001, self.y1, self.z1));
        Some(aabb)
    }
}