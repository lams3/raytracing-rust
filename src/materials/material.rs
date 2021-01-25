use crate::structures::{Ray, Color, HitRecord};

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}