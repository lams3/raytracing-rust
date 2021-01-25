use crate::structures::Vec3;

use rand::prelude::{thread_rng, Rng};

pub type Color = Vec3;

impl Color {
    pub fn to_pixel(&self) -> [u8; 3] {
        let mut pixel = [0 as u8; 3];
        
        for i in 0..3 {
            pixel[i] = f64::min(255.999 * self[i].sqrt(), 255.0) as u8
        }

        pixel
    }

    pub fn random(from: f64, to: f64) -> Self {
        let mut rng = thread_rng();
        Color::new(rng.gen_range(from..to), rng.gen_range(from..to), rng.gen_range(from..to))
    }
}