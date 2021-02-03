pub trait Remapable : Sized {
    type Output;

    fn remap(&self, from: (Self, Self), to: (Self::Output, Self::Output)) -> Self::Output;
}

impl Remapable for f64 {
    type Output = f64;
    
    fn remap(&self, from: (f64, f64), to: (f64, f64)) -> Self::Output {
        to.0 + (self - from.0) * (to.1 - to.0) / (from.1 - from.0)
    }
}

pub trait InverseLerp {
    fn inverse_lerp(a: Self, b: Self, v: Self) -> f64;
}

impl InverseLerp for f64 {
    fn inverse_lerp(a: Self, b: Self, v: Self) -> f64 {
        (v - a) / (b - a)
    }
}