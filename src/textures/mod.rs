pub mod texture;
pub use self::texture::Texture;

pub mod sampling_mode;
pub use self::sampling_mode::SamplingMode;

pub mod solid_color;
pub use self::solid_color::SolidColor;

pub mod checker;
pub use self::checker::Checker;

pub mod noise;
pub use self::noise::Noise;

pub mod image_texture;
pub use self::image_texture::ImageTexture;