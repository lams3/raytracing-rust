use crate::hittables::{Hittable};
use crate::structures::{Vec3, Point3, Ray, HitRecord};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center: center,
            radius: radius
        }
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
                let n = p - self.center;
                return Some(HitRecord::new(p, n, hit))
            }
        }
        
        return None;
    }
}