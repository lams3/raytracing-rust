use crate::structures::{Color, Vec3, Ray, HitRecord};
use crate::materials::Material;

pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self{
        Self {
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = Vec3::random_in_hemisphere(&hit.normal);
        let scattered_ray = Ray::with_time(hit.point, scatter_direction, ray.time);
        let attenuation = self.albedo;
        Some((scattered_ray, attenuation))
    }
}