use crate::structures::{Vec3, Point3, Ray};
use crate::materials::Material;

#[derive(Clone, Copy, Default)]
pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Option<&'a dyn Material>,
    pub t: f64,
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Point3, normal: Vec3, material: &'a dyn Material, t: f64) -> Self {
        Self {
            point: point,
            normal: normal.normalized(),
            material: Some(material),
            t: t
        }
    }

    pub fn is_front_facing(&self, ray: &Ray) -> bool {
        Vec3::dot(&self.normal, &ray.direction) < 0.0
    }
}