use crate::structures::{Color, Ray, Image};
use crate::hittables::Hittable;
use crate::rendering::Camera;
use crate::rendering::skyboxes::Skybox;

use std::f64::INFINITY;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::channel;

use rand::prelude::thread_rng;
use rand::distributions::{Distribution, Uniform};

use scoped_threadpool::Pool;

pub struct RenderParams {
    pub image_width: usize,
    pub image_height: usize,
    pub num_samples: u32,
    pub max_ray_depth: u32,
}

pub fn render<'a, T: FnMut(u32, u32) + 'a>(world: Arc<dyn Hittable>, skybox: Arc<dyn Skybox>, camera: Arc<Camera>, params: &RenderParams, progress: T) -> Image {
    let mut progress = progress;
    let thread_count = get_thread_count();
    let mut pool = Pool::new(thread_count);
    let (tx, rx) = channel();
    
    let final_image = Arc::new(Mutex::new(Image::new(params.image_width, params.image_height)));
    
    pool.scoped(|scoped| {
        for _ in 0..params.num_samples {
            let tx = tx.clone();
            let world = world.clone();
            let skybox = skybox.clone();
            let camera = camera.clone();
            let final_image = final_image.clone();
            
            scoped.execute(move || {
                let sample = render_sample(world, skybox, camera, params);
                let mut render = final_image.lock().unwrap();
                *render += sample / params.num_samples as f64;
                tx.send(()).unwrap();
            });
        }

        for i in 0..params.num_samples {
            rx.recv().unwrap();
            (&mut progress)(i + 1, params.num_samples);
        }
    });

    let final_image = (*final_image.lock().unwrap()).clone();
    final_image
}

fn render_sample(world: Arc<dyn Hittable>, skybox: Arc<dyn Skybox>, camera: Arc<Camera>, params: &RenderParams) -> Image {
    let mut rng = thread_rng();
    let uniform_distribution = Uniform::from(-0.5..=0.5);
    let mut image = Image::new(params.image_width, params.image_height);

    for x in 0..params.image_width {
        for y in 0..params.image_height {
            let fx = (x as f64 + 0.5) + uniform_distribution.sample(&mut rng);
            let fy = (y as f64 + 0.5) + uniform_distribution.sample(&mut rng);

            let u = fx as f64 / params.image_width as f64;
            let v = 1.0 - (fy as f64 / params.image_height as f64);

            let ray = camera.get_ray(u, v);
            let color = ray_color(&ray, world.clone(), skybox.clone(), params.max_ray_depth);

            image[(x, y)] = color;
        }
    }

    image
}

fn ray_color(ray: &Ray, world: Arc<dyn Hittable>, skybox: Arc<dyn Skybox>, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    
    match world.hit(&ray, 0.001, INFINITY) {
        Some(hit) => match hit.material.unwrap().scatter(ray, &hit) {
            Some((scattered_ray, attenuation)) => attenuation * ray_color(&scattered_ray, world, skybox, depth - 1),
            None => Color::new(0.0, 0.0, 0.0)
        }
        None => skybox.get_color(&ray)
    }
}

fn get_thread_count() -> u32 {
    num_cpus::get_physical() as u32
}