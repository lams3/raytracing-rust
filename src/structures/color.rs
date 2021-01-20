use crate::structures::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn to_pixel(&self) -> [u8; 3] {
        let mut pixel = [0 as u8; 3];
        
        for i in 0..3 {
            pixel[i] = f64::min(255.999 * self[i], 255.0) as u8
        }

        pixel
    }
}