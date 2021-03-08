use crate::textures::Texture;
use crate::structures::{Point3, Color};

pub struct SolidColor {
    pub color: Color
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self {
            color: color
        }
    }
}

impl Texture for SolidColor {

    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        self.color
    }
}