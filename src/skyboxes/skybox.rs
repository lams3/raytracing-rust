use crate::structures::{Ray, Color};

pub trait Skybox: Send + Sync {
    fn get_color(&self, ray: &Ray) -> Color;
}