use crate::structures::HitRecord;
use crate::structures::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<T: Hittable> Hittable for &T {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (*self).hit(&ray, t_min, t_max)
    }
}

impl<T: Hittable> Hittable for &mut T {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (**self).hit(&ray, t_min, t_max)
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut current_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in self {
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