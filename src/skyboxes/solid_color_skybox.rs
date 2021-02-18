use crate::structures::{Ray, Color};
use crate::skyboxes::Skybox;

pub struct SolidColorSkybox {
    pub color: Color,
}

impl SolidColorSkybox {
    pub fn new(color: Color) -> Self {
        Self {
            color: color
        }
    }
}

impl Skybox for SolidColorSkybox {
    fn get_color(&self, _ray: &Ray) -> Color {
        self.color
    }
}