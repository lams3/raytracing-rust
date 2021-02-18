use crate::structures::{AABB, HitRecord, Point3, Ray, Vec3};
use crate::hittables::Hittable;
use crate::materials::Material;
use crate::utility::InverseLerp;

use std::sync::Arc;

pub struct XYRect {
    pub material: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub z: f64
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, z: f64, material: Arc<dyn Material>) -> Self {
        Self {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
            z: z,
            material: material
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.z - ray.origin.z) / ray.direction.z;
        
        if t < t_min || t > t_max {
            return None
        }

        let p = ray.at(t);

        if (p.x < self.x0 || p.x > self.x1) || (p.y < self.y0 || p.y > self.y1) {
            return None
        }

        let u = f64::inverse_lerp(self.x0, self.x1, p.x);
        let v = f64::inverse_lerp(self.y0, self.y1, p.y);

        Some(HitRecord::new(p, Vec3::front(), self.material.clone(), t, u, v))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        let aabb = AABB::new(Point3::new(self.x0, self.y0, self.z - 0.0001), Point3::new(self.x1, self.y1, self.z + 0.0001));
        Some(aabb)
    }
}