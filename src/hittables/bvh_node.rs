use crate::structures::{AABB, HitRecord, Ray};
use crate::hittables::{Hittable, HittableList};

use std::sync::Arc;
use std::cmp::Ordering;
use std::cmp::Ordering::Less;

use rand::{thread_rng, Rng};

pub struct BVHNode {
    pub aabb: AABB,
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
}

impl BVHNode {
    pub fn new(hittable_list: &HittableList, time_0: f64, time_1: f64) -> Self {
        let objects: Vec<Arc<dyn Hittable>> = hittable_list.hittables[0..hittable_list.hittables.len()].iter().cloned().collect();
        Self::internal_new(&objects, time_0, time_1)
    }

    fn internal_new(objects: &Vec<Arc<dyn Hittable>>, time_0: f64, time_1: f64) -> Self {
        let mut rng = thread_rng();

        let mut objects = objects.clone();

        let axis = rng.gen_range(0..3);
        let comparator = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| BVHNode::box_compare(a, b, axis, time_0, time_1);

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if objects.len() == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if objects.len() == 2 {
            if comparator(&objects[0], &objects[1]) == Less {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                left = objects[1].clone();
                right = objects[0].clone();
            }
        } else {
            objects.sort_by(comparator);
            let mid = objects.len() / 2;
            let l_vec: Vec<Arc<dyn Hittable>> = objects[0..mid].iter().cloned().collect();
            let r_vec: Vec<Arc<dyn Hittable>> = objects[mid..objects.len()].iter().cloned().collect();
            left = Arc::new(Self::internal_new(&l_vec, time_0, time_1));
            right = Arc::new(Self::internal_new(&r_vec, time_0, time_1));
        }

        let aabb = match (left.bounding_box(time_0, time_1), right.bounding_box(time_0, time_1)) {
            (Some(l_box), Some(r_box)) => l_box + r_box,
            _ => panic!("Hittable with no bounding box passed into BVHNode constructor.")
        };

        Self {
            aabb: aabb,
            left: left,
            right: right
        }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize, time_0: f64, time_1: f64) -> Ordering {
        match (a.bounding_box(time_0, time_1), b.bounding_box(time_0, time_1)) {
            (Some(box_a), Some(box_b)) => box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap(),
            _ => panic!("Hittable with no bounding box passed into BVHNode constructor.")
        }
    }
}

impl Hittable for BVHNode {
    
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(*ray, t_min, t_max) {
            return None
        }

        match self.left.hit(ray, t_min, t_max) {
            Some(record) => match self.right.hit(ray, t_min, record.t) {
                Some(hit) => Some(hit),
                None => Some(record) 
            }
            None => self.right.hit(ray, t_min, t_max)
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.aabb)
    }
}