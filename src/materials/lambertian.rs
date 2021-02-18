use crate::structures::{Color, Vec3, Ray, HitRecord};
use crate::textures::Texture;
use crate::materials::Material;

use std::sync::Arc;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self {
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = Vec3::random_in_hemisphere(&hit.normal);
        let scattered_ray = Ray::with_time(hit.point, scatter_direction, ray.time);
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        Some((scattered_ray, attenuation))
    }
}