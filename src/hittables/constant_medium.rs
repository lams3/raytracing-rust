use crate::structures::{ Vec3, Ray, HitRecord, AABB };
use crate::hittables::Hittable;
use crate::textures::Texture;
use crate::materials::Material;
use crate::materials::volumetric::Isotropic;

use std::sync::Arc;
use rand::{ thread_rng, Rng };

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    pub density: f64
}

impl Hittable for ConstantMedium {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> { 
        let mut rng = thread_rng();

        let (mut entry_hit, mut exit_hit): (HitRecord, HitRecord);

        match self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY) {
            Some(hit) => entry_hit = hit,
            None => return None
        };

        match self.boundary.hit(ray, entry_hit.t + 0.0001, f64::INFINITY) {
            Some(hit) => exit_hit = hit,
            None => return None
        };

        if entry_hit.t < t_min {
            entry_hit.t = t_min;
        }

        if exit_hit.t > t_max {
            exit_hit.t = t_max;
        }

        if entry_hit.t >= exit_hit.t {
            return None;
        }

        if entry_hit.t < 0.0 {
            entry_hit.t = 0.0;
        }

        let ray_lenght = ray.direction.length();
        let distance_inside_boundary = (exit_hit.t - entry_hit.t) * ray_lenght;
        let hit_distance = (-1.0 / self.density) * f64::ln(rng.gen());
    
        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = entry_hit.t + (hit_distance / ray_lenght);
        let hit = HitRecord::new(ray.at(t), Vec3::up(), self.phase_function.clone(), t, 0.0, 0.0);

        Some(hit)
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> { 
        self.boundary.bounding_box(time_0, time_1)
    }
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, albedo: Arc<dyn Texture>) -> Self {
        Self {
            boundary: boundary,
            density: density,
            phase_function: Arc::new(Isotropic::new(albedo))
        }
    }
}