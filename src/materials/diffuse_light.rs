use crate::structures::{Color, Point3, Ray, HitRecord};
use crate::textures::Texture;
use crate::materials::Material;

use std::sync::Arc;

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self {
            emit: emit
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
