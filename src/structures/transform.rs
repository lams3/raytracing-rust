use crate::structures::{Vec3, Point3, Quaternion, Ray};

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quaternion,
    pub scale: Vec3
}

impl Transform {
    pub fn new(translation: Vec3, rotation: Quaternion, scale: Vec3) -> Self {
        Self {
            translation: translation,
            rotation: rotation,
            scale: scale
        }
    }

    pub fn transform_ray(&self, ray: Ray) -> Ray {
        Ray::with_time(
            self.transform_point(ray.origin), 
            self.transform_vector(ray.direction), 
            ray.time
        )
    }

    pub fn transform_vector(&self, vec: Vec3) -> Vec3 {
        self.rotation.rotate_vector(vec * self.scale)
    }

    pub fn transform_point(&self, point: Point3) -> Point3 {
        self.transform_vector(point) + self.translation
    }

    pub fn inverse_transform_ray(&self, ray: Ray) -> Ray {
        Ray::with_time(
            self.inverse_transform_point(ray.origin), 
            self.inverse_transform_vector(ray.direction), 
            ray.time
        )
    }

    pub fn inverse_transform_vector(&self, vec: Vec3) -> Vec3 {
        let mut inv_scale = self.scale;
        for i in 0..3 {
            inv_scale[i] = 1.0 / inv_scale[i];
        }

        self.rotation.inverse().rotate_vector(vec) * inv_scale
    }

    pub fn inverse_transform_point(&self, point: Point3) -> Point3 {
        self.inverse_transform_vector(point - self.translation)
    }

    pub fn interpolate(a: Self, b: Self, t: f64) -> Self {
        Self {
            translation: Vec3::lerp(&a.translation, &b.translation, t),
            rotation: Quaternion::slerp(a.rotation, b.rotation, t),
            scale: Vec3::lerp(&a.scale, &b.scale, t)
        }
    }
}