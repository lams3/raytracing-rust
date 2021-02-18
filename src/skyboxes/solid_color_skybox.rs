use crate::structures::{Ray, Color, Vec3};
use crate::utility::Remapable;
use crate::skyboxes::Skybox;

pub struct SolidColorSkybox {
    pub color: Color,
}

impl GradientSkybox {
    pub fn new(color: Color) -> Self {
        Self {
            color: color
        }
    }
}

impl Skybox for GradientSkybox {
    fn get_color(&self, ray: &Ray) -> Color {
        self->color
    }
}