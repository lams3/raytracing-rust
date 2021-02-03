use crate::structures::{Ray, HitRecord, Transform, AABB};
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

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        match self.hittable.bounding_box(time_0, time_1) {
            Some(aabb) => {
                let points = aabb.get_points().iter().map(|&el| self.transform.transform_point(el)).collect();
                Some(AABB::from_points(&points))
            },
            None => None
        }
    }
}