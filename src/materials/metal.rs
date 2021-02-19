use crate::textures::Texture;
use crate::structures::{Color, Vec3, Ray, HitRecord};
use crate::materials::Material;

use std::sync::Arc;

pub struct Metal {
    pub albedo: Arc<dyn Texture>,
    pub fuzziness: f64
}

impl Metal {
    pub fn new(albedo: Arc<dyn Texture>, fuzziness: f64) -> Self{
        Self {
            albedo: albedo,
            fuzziness: fuzziness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let normal = hit.get_facing_normal(ray);
        let reflected = Vec3::reflect(&ray.direction, &normal);
        let scatter_direction = reflected + self.fuzziness * Vec3::random_in_unit_sphere();
        let scattered_ray = Ray::with_time(hit.point, scatter_direction, ray.time);
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);

        if Vec3::dot(&scattered_ray.direction, &normal) > 0.0 {
            Some((scattered_ray, attenuation))
        } else {
            None
        }
    }
}