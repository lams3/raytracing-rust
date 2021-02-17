use crate::structures::{Point3, Color};
use crate::textures::Texture;
use super::Perlin;

pub struct Noise {
    pub scale: f64,
    perlin: Perlin
}

impl Noise {
    pub fn new(scale: f64) -> Self {
        Self {
            scale: scale,
            perlin: Perlin::new()
        }
    }
}

impl Texture for Noise {

    fn value(&self, _: f64, _: f64, p: Point3) -> Color {
        let turbulence = self.perlin.turbulence(p);
        let noise = 0.5 * (1.0 + f64::sin(self.scale * p.z + 10.0 * turbulence));
        Color::new(1.0, 1.0, 1.0) * noise
    }
}