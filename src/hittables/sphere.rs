use crate::structures::{Vec3, Point3, Ray, HitRecord, AABB};
use crate::hittables::Hittable;
use crate::materials::Material;

use std::sync::Arc;
use std::f64::consts::PI;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material
        }
    }

    fn calc_uv(&self, p: Point3) -> (f64, f64) {
        let vec = (p - self.center).normalized();
        let phi = f64::atan2(-vec.z, vec.x) + PI;
        let theta = f64::acos(-vec.y);
        let u = phi / (2.0 * PI);
        let v = theta / PI;
       
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        
        let half_b = Vec3::dot(&ray.direction, &oc);
        let c = oc.squared_length() - self.radius.powi(2);

        let delta = half_b.powi(2) - c;

        if delta < 0.0 {
            return None;
        }

        for signal in &[-1.0, 1.0] {
            let hit = -half_b + signal * delta.sqrt();
            
            if t_min < hit && hit < t_max {
                let p = ray.at(hit);
                let n = (p - self.center) / self.radius;
                let (u, v) = self.calc_uv(p);
                return Some(HitRecord::new(p, n, self.material.clone(), hit, u, v))
            }
        }
        
        return None;
    }
    
    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let vec = self.radius * Vec3::new(1.0, 1.0, 1.0);
        Some(AABB::new(self.center - vec, self.center + vec))
    }
}