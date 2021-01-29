use crate::structures::{Ray, HitRecord};
use crate::hittables::Hittable;

use std::sync::Arc;

pub struct HittableList {
    hittables: Vec<Arc<dyn Hittable>>
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
}