use crate::structures::{Ray, HitRecord, Transform};
use crate::hittables::Hittable;

use std::sync::Arc;

pub struct Instance {
    pub hittable: Arc<dyn Hittable>,
    pub transform: Transform
}

impl Instance {
    pub fn new(hittable: Arc<dyn Hittable>, transform: Transform) -> Self {
        Self {
            hittable: hittable,
            transform: transform
        }
    }
}

impl Hittable for Instance {
    
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ray = self.transform.inverse_transform_ray(*ray);
        self.hittable.hit(&ray, t_min, t_max)
    }
}