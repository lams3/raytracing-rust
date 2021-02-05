use crate::structures::{Point3, Ray, HitRecord, AABB};
use crate::hittables::Hittable;

use std::sync::Arc;

pub struct HittableList {
    pub hittables: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittables: vec![]
        }
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.hittables.push(hittable);
    }

    pub fn remove(&mut self, hittable: Arc<dyn Hittable>) {
        self.hittables.retain(|x| !Arc::ptr_eq(&hittable, &x));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut current_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in &self.hittables {
            match hittable.hit(ray, t_min, closest_so_far) {
                Some(record) => {
                    current_hit = Some(record);
                    closest_so_far = record.t
                },
                None => ()
            }
        }

        current_hit
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        if self.hittables.is_empty() {
            return None
        }

        let mut total = AABB::new(Point3::zero(), Point3::zero());
        let mut first_box = true;

        for hittable in &self.hittables {
            match hittable.bounding_box(time_0, time_1) {
                Some(aabb) => {
                    if first_box {
                        first_box = false;
                        total = aabb;
                    } else {
                        total.encapsulate(aabb);
                    }
                },
                None => return None
            }
        }

        Some(total)
    }
}