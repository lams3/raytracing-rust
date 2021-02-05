use crate::structures::{Ray, HitRecord, Transform, AABB};
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

    pub fn transform_at(&self, time: f64) -> Transform {
        let t = f64::inverse_lerp(self.time_0, self.time_1, time);
        Transform::interpolate(self.transform_0, self.transform_1, t)
    }
}

impl Hittable for MovingInstance {
    
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let transform = self.transform_at(ray.time);
        let ray = transform.inverse_transform_ray(*ray);
        match self.hittable.hit(&ray, t_min, t_max) {
            Some(mut record) => {
                record.point = transform.transform_point(record.point);
                record.normal = transform.transform_vector(record.normal);
                Some(record)
            },
            None => None
        }
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        match self.hittable.bounding_box(time_0, time_1) {
            Some(aabb) => {
                let transform_0 = self.transform_at(time_0);
                let transform_1 = self.transform_at(time_1);
                let mut points_0 = aabb.get_points().iter().map(|&el| transform_0.transform_point(el)).collect();
                let mut points_1 = aabb.get_points().iter().map(|&el| transform_1.transform_point(el)).collect();
                let mut points = Vec::with_capacity(16);
                points.append(&mut points_0);
                points.append(&mut points_1);
                Some(AABB::from_points(&points))
            }
            None => None,
        }
    }
}