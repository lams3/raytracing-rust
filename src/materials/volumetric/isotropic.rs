use crate::structures::{ Vec3, Ray, Color, HitRecord };
use crate::materials::Material;
use crate::textures::Texture;

use std::sync::Arc;

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>
}

impl Material for Isotropic {

    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> { 
        let scattered_ray = Ray::with_time(hit.point, Vec3::random_in_unit_sphere(), ray.time);
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        Some((scattered_ray, attenuation))
    }

}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self {
            albedo: albedo
        }
    }
}