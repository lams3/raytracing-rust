use crate::structures::{Point3, Ray};

pub struct AABB {
    pub min: Point3,
    pub max: Point3
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self {
            min: min,
            max: max
        }
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for i in 0..3 {
            let v_min = (self.min[i] - ray.origin[i]) / ray.direction[i];
            let v_max = (self.max[i] - ray.origin[i]) / ray.direction[i];

            let t0 = f64::min(v_min, v_max);
            let t1 = f64::max(v_min, v_max);

            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);

            if t_max <= t_min {
                return false
            }
        }

        true
    }
}