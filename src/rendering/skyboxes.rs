use crate::structures::{Ray, Color, Vec3};
use crate::utility::Remapable;

pub trait Skybox: Send + Sync {
    fn get_color(&self, ray: &Ray) -> Color;
}

pub struct GradientSkybox {
    pub from: Color,
    pub to: Color,
    pub direction: Vec3
}

impl GradientSkybox {
    pub fn new(from: Color, to: Color, direction: Vec3) -> Self {
        Self {
            from: from,
            to: to,
            direction: direction
        }
    }
}

impl Skybox for GradientSkybox {
    fn get_color(&self, ray: &Ray) -> Color {
        let t = Vec3::dot(&ray.direction, &self.direction).remap((-1.0, 1.0), (0.0, 1.0));
        Vec3::lerp(&self.from, &self.to, t)
    }
}