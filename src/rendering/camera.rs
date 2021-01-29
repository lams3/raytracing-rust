use crate::structures::{Point3, Vec3, Ray};

pub struct Camera {
    pub vertical_fov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_dist: f64,
    pub position: Point3,
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, up: Vec3, vertical_fov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let z_axis = (look_from - look_at).normalized();
        let x_axis = Vec3::cross(&up, &z_axis).normalized();
        let y_axis = Vec3::cross(&z_axis, &x_axis).normalized();

        Self {
            position: look_from,
            vertical_fov: vertical_fov,
            aspect_ratio: aspect_ratio,
            aperture: aperture,
            focus_dist: focus_dist,
            x_axis: x_axis,
            y_axis: y_axis,
            z_axis: z_axis
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let viewport_height = 2.0 * (self.vertical_fov / 2.0).tan();
        let viewport_width = viewport_height * self.aspect_ratio;

        let horizontal = self.focus_dist * viewport_width * self.x_axis;
        let vertical = self.focus_dist * viewport_height * self.y_axis;
        let lower_left_corner = self.position - (horizontal / 2.0) - (vertical / 2.0) - self.focus_dist * self.z_axis;


        let lens_radius = self.aperture / 2.0;
        let sample_lens = lens_radius * Vec3::random_in_unit_sphere();
        let ray_origin = self.position + sample_lens.x * self.x_axis + sample_lens.y * self.y_axis;
        let ray_direction = lower_left_corner + (u * horizontal) + (v * vertical) - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}