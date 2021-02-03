use crate::structures::{Color, Vec3, Ray, HitRecord};
use crate::materials::Material;

use rand::prelude::{thread_rng, Rng};

pub struct Dieletric {
    pub refraction_index: f64
}

impl Dieletric {
    pub fn new(refraction_index: f64) -> Self{
        Self {
            refraction_index: refraction_index
        }
    }

    fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = f64::powi(r0, 2);
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dieletric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let mut rng = thread_rng();

        let attenuation = Color::new(1.0, 1.0, 1.0);

        let (eta_in, eta_out, normal) = match hit.is_front_facing(ray) {
            true => (1.0, self.refraction_index, hit.normal),
            false => (self.refraction_index, 1.0, -hit.normal)
        };

        let refraction_ratio = eta_in / eta_out;
        let cos_theta = f64::min(Vec3::dot(&(-ray.direction), &normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - f64::powi(cos_theta, 2));
        let cant_refract = sin_theta * refraction_ratio > 1.0;
  
        let scatter_direction = if cant_refract || Dieletric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
            Vec3::reflect(&ray.direction, &normal)
        } else {
            Vec3::refract(&ray.direction, &normal, eta_in, eta_out)
        };

        
        let scattered_ray = Ray::with_time(hit.point, scatter_direction, ray.time);
        
        Some((scattered_ray, attenuation))
    }
}