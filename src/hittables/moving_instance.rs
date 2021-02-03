use crate::structures::{Ray, HitRecord, Transform};
use crate::hittables::Hittable;
use crate::utility::InverseLerp;

use std::sync::Arc;

pub struct MovingInstance {
    pub hittable: Arc<dyn Hittable>,
    pub transform_0: Transform,
    pub transform_1: Transform,
    pub time_0: f64,
    pub time_1: f64
}

impl MovingInstance {
    pub fn new(hittable: Arc<dyn Hittable>, transform_0: Transform, transform_1: Transform, time_0: f64, time_1: f64) -> Self {
        Self {
            hittable: hittable,
            transform_0: transform_0,
            transform_1: transform_1,
            time_0: time_0,
            time_1: time_1
        }
    }
}

impl Hittable for MovingInstance {
    
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = f64::inverse_lerp(self.time_0, self.time_1, ray.time);
        let transform = Transform::interpolate(self.transform_0, self.transform_1, t);
        let ray = transform.inverse_transform_ray(*ray);
        self.hittable.hit(&ray, t_min, t_max)
    }
}