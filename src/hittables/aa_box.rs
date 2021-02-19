use crate::structures::{Point3, Ray, HitRecord, AABB};
use crate::hittables::{Hittable, HittableList, XYRect, XZRect, YZRect};
use crate::materials::Material;

use std::sync::Arc;

pub struct AABox {
    pub min: Point3,
    pub max: Point3,
    pub material: Arc<dyn Material>,
    sides: HittableList,
}

impl AABox {
    pub fn new(min: Point3, max: Point3, material: Arc<dyn Material>) -> Self {
        let mut new = Self {
            min: min,
            max: max,
            material: material,
            sides: HittableList::new()
        };

        new.generate_sides();

        new
    }

    pub fn generate_sides(&mut self) {
        self.sides.hittables.clear();

        self.sides.add(Arc::new(XYRect::new(self.min.x, self.max.x, self.min.y, self.max.y, self.min.z, self.material.clone())));
        self.sides.add(Arc::new(XYRect::new(self.min.x, self.max.x, self.min.y, self.max.y, self.max.z, self.material.clone())));

        self.sides.add(Arc::new(XZRect::new(self.min.x, self.max.x, self.min.z, self.max.z, self.min.y, self.material.clone())));
        self.sides.add(Arc::new(XZRect::new(self.min.x, self.max.x, self.min.z, self.max.z, self.max.y, self.material.clone())));

        self.sides.add(Arc::new(YZRect::new(self.min.y, self.max.y, self.min.z, self.max.z, self.min.x, self.material.clone())));
        self.sides.add(Arc::new(YZRect::new(self.min.y, self.max.y, self.min.z, self.max.z, self.max.x, self.material.clone())));
    }
}

impl Hittable for AABox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}