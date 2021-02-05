use crate::structures::{Ray, HitRecord, AABB};

pub trait Hittable: Send + Sync {
    
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB>;
}