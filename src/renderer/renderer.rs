use crate::structures::{Color, Ray};
use crate::renderer::Camera;
use crate::renderer::skybox::{Skybox};
use crate::hittables::{Hittable};

use std::f64::INFINITY;
use rand::prelude::thread_rng;
use rand::distributions::{Distribution, Uniform};

pub struct Renderer {
    pub image_width: u32,
    pub image_height: u32,
    pub num_samples: u32,
    pub max_depth: u32,
    pub buffer: Vec<u8>
}

impl Renderer {    
    pub fn new(image_width: u32, image_height: u32, num_samples: u32, max_depth: u32) -> Self {
        Self {
            image_width: image_width,
            image_height: image_height,
            num_samples: num_samples,
            max_depth: max_depth,
            buffer: vec![0; (3 * image_width * image_height) as usize]
        }
    }

    pub fn render<'a, T: FnMut(f64) + 'a>(&mut self, path: &str, world: &dyn Hittable, skybox: &dyn Skybox, progress: T) {
        let mut clojure = progress;
        
        let mut rng = thread_rng();
        let uniform_distrbution = Uniform::from(-0.5..=0.5);

        let camera = Camera::new(Default::default(), (120.0 as f64).to_radians(), self.aspect_ratio());

        for x in 0..self.image_width {
            (&mut clojure)(x as f64 / self.image_width as f64);

            for y in 0..self.image_height {

                let mut color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.num_samples {
                    let fx = (x as f64 + 0.5) + uniform_distrbution.sample(&mut rng);
                    let fy = (y as f64 + 0.5) + uniform_distrbution.sample(&mut rng);
    
                    let u = fx as f64 / self.image_width as f64;
                    let v = 1.0 - (fy as f64 / self.image_height as f64);
    
                    let ray = camera.get_ray(u, v);
                    color += self.ray_color(&ray, world, skybox, self.max_depth);
                }
                
                let pixel = (color / self.num_samples as f64).to_pixel();

                let index = (3 * (y * self.image_width + x)) as usize;
                for i in 0..3 {
                    self.buffer[index + i] = pixel[i];
                }
            }
        }

        image::save_buffer(path, &self.buffer, self.image_width, self.image_height, image::ColorType::Rgb8).unwrap();

        (&mut clojure)(1.0);
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable, skybox: &dyn Skybox, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        
        match world.hit(&ray, 0.001, INFINITY) {
            Some(hit) => match hit.material.unwrap().scatter(ray, &hit) {
                Some((scattered_ray, attenuation)) => attenuation * self.ray_color(&scattered_ray, world, skybox, depth - 1),
                None => Color::new(0.0, 0.0, 0.0)
            }
            None => skybox.get_color(&ray)
        }
    }

    fn aspect_ratio(&self) -> f64 {
        self.image_width as f64 / self.image_height as f64
    }
}