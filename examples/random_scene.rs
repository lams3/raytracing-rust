extern crate raytracer;

use pbr::ProgressBar;

use raytracer::structures::Vec3;
use raytracer::hittables::Sphere;
use raytracer::renderer::Renderer;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 144;
const N_SAMPLES: u32 = 100;

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let mut progress_bar = ProgressBar::new(100);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = vec![&sphere, &ground];

    renderer.render("./output.png", &world, N_SAMPLES, move |current_progress| {
        progress_bar.set((current_progress * 100.0) as u64);
    });
}