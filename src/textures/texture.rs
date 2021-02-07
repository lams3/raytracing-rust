use crate::structures::{Point3, Color};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}