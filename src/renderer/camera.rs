use crate::structures::{Vec3, Ray};

pub struct Camera {
    pub position: Vec3,
    pub vertical_fov: f64,
    pub aspect_ratio: f64
}

impl Camera {
    pub fn new(position: Vec3, vertical_fov: f64, aspect_ratio: f64) -> Self {
        Self {
            position: position,
            vertical_fov: vertical_fov,
            aspect_ratio: aspect_ratio
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let viewport_height = (self.vertical_fov / 2.0).tan();
        let viewport_width = viewport_height * self.aspect_ratio;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = self.position - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, 1.0);

        Ray::new(self.position, lower_left_corner + u * horizontal + v * vertical)
    }
}