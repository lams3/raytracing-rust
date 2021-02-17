use crate::structures::Color;

use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign, Index, IndexMut};

use image::GenericImageView;

#[derive(Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            buffer: vec![Default::default(); width * height]
        }
    }

    pub fn read(path: &str) -> Self {
        let loaded = image::open(path).unwrap();
        
        let (width, height) = loaded.dimensions();
        let mut image = Self::new(width as usize, height as usize);
        
        for (x, y, p) in loaded.pixels() {
            let color = Color::new((p[0] as f64) / 255.0, (p[1] as f64) / 255.0, (p[2] as f64) / 255.0);
            image[(x as usize, y as usize)] = color;
        }

        image
    }

    pub fn save(&self, path: &str) {
        let mut u8_buffer: Vec<u8> = vec![0; (3 * self.width * self.height) as usize];
        
        for i in 0..self.buffer.len() {
            let pixel = self.buffer[i].to_pixel();
            
            for j in 0..3 {
                u8_buffer[3 * i + j] = pixel[j];
            }
        }

        image::save_buffer(path, &u8_buffer, self.width as u32, self.height as u32, image::ColorType::Rgb8).unwrap();
    }
}

impl Neg for Image {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            width: self.width,
            height: self.height,
            buffer: self.buffer.iter().map(|&el| -el).collect()
        }
    }
}

impl Add for Image {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);
        
        Self {
            width: self.width,
            height: self.height,
            buffer: self.buffer.iter().enumerate().map(|(i, &el)| el + other.buffer[i]).collect()
        }
    }
}

impl AddAssign for Image {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);
        
        for (i, el) in self.buffer.iter_mut().enumerate() {
            *el += other.buffer[i];
        }
    }
}

impl Sub for Image {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);
        
        Self {
            width: self.width,
            height: self.height,
            buffer: self.buffer.iter().enumerate().map(|(i, &el)| el - other.buffer[i]).collect()
        }
    }
}

impl SubAssign for Image {
    fn sub_assign(&mut self, other: Self) {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);
        
        for (i, el) in self.buffer.iter_mut().enumerate() {
            *el -= other.buffer[i];
        }
    }
}

impl Div<f64> for Image {
    type Output = Self;

    fn div(self, factor: f64) -> Self::Output {
        Self {
            width: self.width,
            height: self.height,
            buffer: self.buffer.iter().map(|&el| el / factor).collect()
        }
    }
}

impl DivAssign<f64> for Image {
    fn div_assign(&mut self, factor: f64) {
        for el in self.buffer.iter_mut() {
            *el /= factor;
        }
    }
}

impl Mul<Image> for f64 {
    type Output = Image;

    fn mul(self, image: Image) -> Self::Output {
        image * self
    }
}

impl Mul<f64> for Image {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Self {
            width: self.width,
            height: self.height,
            buffer: self.buffer.iter().map(|&el| el * factor).collect()
        }
    }
}

impl MulAssign<f64> for Image {
    fn mul_assign(&mut self, factor: f64) {
        for el in self.buffer.iter_mut() {
            *el *= factor;
        }
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x >= self.width || y >= self.height {
            panic!("index out of range!");
        }
        
        let index = y * self.width + x;
        return &self.buffer[index];
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x >= self.width || y >= self.height {
            panic!("index out of range!");
        }
        
        let index = y * self.width + x;
        return &mut self.buffer[index];
    }
}