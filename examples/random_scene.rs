extern crate raytracer;

use std::rc::Rc;

use pbr::ProgressBar;

use raytracer::structures::{Color, Vec3};
use raytracer::hittables::Sphere;
use raytracer::renderer::Renderer;
use raytracer::renderer::skybox::GradientSkybox;
use raytracer::materials::{Metal, Lambertian, Dieletric};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
const N_SAMPLES: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT, N_SAMPLES, MAX_DEPTH);
    let mut progress_bar = ProgressBar::new(100);


    let skybox = GradientSkybox::new(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), Vec3::new(0.0, 1.0, 0.0));
    let left = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0), 
        0.5,
        Rc::new(Dieletric::new(1.5))
    );
    let left_inner = Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0), 
        -0.4, 
        Rc::new(Dieletric::new(1.5))
    );
    let center = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0), 
        0.5, 
        Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)))
    );
    let right = Sphere::new(
        Vec3::new(1.0, 0.0, -1.0), 
        0.5, 
        Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0))
    );
    let ground = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0), 
        100.0,
        Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)))
    );
    let world = vec![&left, &left_inner, &center, &right, &ground];

    renderer.render("./output.png", &world, &skybox, move |current_progress| {
        progress_bar.set((current_progress * 100.0) as u64);
    });
}