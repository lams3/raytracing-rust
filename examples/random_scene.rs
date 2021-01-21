extern crate raytracer;

use pbr::ProgressBar;

use raytracer::structures::{Color, Vec3};
use raytracer::hittables::Sphere;
use raytracer::renderer::Renderer;
use raytracer::renderer::skybox::GradientSkybox;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
const N_SAMPLES: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT, N_SAMPLES, MAX_DEPTH);
    let mut progress_bar = ProgressBar::new(100);

    let skybox = GradientSkybox::new(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), Vec3::new(0.0, 1.0, 0.0));
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = vec![&sphere, &ground];

    renderer.render("./output.png", &world, &skybox, move |current_progress| {
        progress_bar.set((current_progress * 100.0) as u64);
    });
}