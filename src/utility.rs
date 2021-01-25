pub trait Remapable {
    type Output;

    fn remap(&self, from: (Self::Output, Self::Output), to: (Self::Output, Self::Output)) -> Self::Output;
}

impl Remapable for f64 {
    type Output = f64;
    
    fn remap(&self, from: (f64, f64), to: (f64, f64)) -> Self::Output {
        to.0 + (self - from.0) * (to.1 - to.0) / (from.1 - from.0)
    }
}