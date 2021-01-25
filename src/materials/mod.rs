pub mod material;
pub use self::material::Material;

pub mod lambertian;
pub use self::lambertian::Lambertian;

pub mod metal;
pub use self::metal::Metal;

pub mod dieletric;
pub use self::dieletric::Dieletric;