use crate::structures::{Ray, Color, HitRecord, Point3};

pub trait Material: Sync + Send {
    fn scatter(&self, incident_ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;

    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}