use crate::structures::{Point3, Color};
use crate::textures::{Texture, SamplingMode};

use std::sync::Arc;

pub struct Checker {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
    pub frequency: f64,
    pub sampling_mode: SamplingMode
}

impl Checker {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>, frequency: f64) -> Self {
        Self {
            odd: odd,
            even: even,
            frequency: frequency,
            sampling_mode: SamplingMode::SOLID
        }
    }

    pub fn with_sampling_mode(odd: Arc<dyn Texture>, even: Arc<dyn Texture>, frequency: f64, sampling_mode: SamplingMode) -> Self {
        Self {
            odd: odd,
            even: even,
            frequency: frequency,
            sampling_mode: sampling_mode
        }
    }

    fn get_sines(&self, u: f64, v: f64, p: Point3) -> f64 {
        match self.sampling_mode {
            SamplingMode::SOLID => f64::sin(self.frequency * p.x) * f64::sin(self.frequency * p.y) * f64::sin(self.frequency * p.z),
            SamplingMode::UV => f64::sin(self.frequency * u) * f64::sin(self.frequency * v)
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = self.get_sines(u, v, p);
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}