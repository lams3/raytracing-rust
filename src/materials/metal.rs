use crate::structures::{Color, Vec3, Ray, HitRecord};
use crate::materials::Material;

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self{
        Self {
            albedo: albedo,
            fuzziness: fuzziness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&ray.direction, &hit.normal);
        let scatter_direction = reflected + self.fuzziness * Vec3::random_in_unit_sphere();
        let scattered_ray = Ray::with_time(hit.point, scatter_direction, ray.time);
        let attenuation = self.albedo;

        if Vec3::dot(&scattered_ray.direction, &hit.normal) > 0.0 {
            Some((scattered_ray, attenuation))
        } else {
            None
        }
    }
}