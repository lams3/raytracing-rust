use crate::structures::{Point3, Color, Image};
use crate::textures::Texture;
use crate::utility::Clamp;

use std::sync::Arc;

pub struct ImageTexture {
    pub image: Arc<Image>
}

impl ImageTexture {
    pub fn new(image: Arc<Image>) -> Self {
        Self {
            image: image
        }
    }

    pub fn read(path: &str) -> Self {
        Self {
            image: Arc::new(Image::read(path))
        }
    }

    pub fn sample(&self, u: f64, v: f64) -> Color {
        let u = Clamp::clamp(u, 0.0, 1.0);
        let v = 1.0 - Clamp::clamp(v, 0.0, 1.0);

        let i = u * (self.image.width as f64);
        let j = v * (self.image.height as f64);

        let i = usize::min(i as usize, self.image.width - 1);
        let j = usize::min(j as usize, self.image.height - 1);

        self.image[(i, j)]
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Point3) -> Color {
        self.sample(u, v)
    }
}