extern crate raytracer;

use raytracer::renderer::Renderer;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const N_SAMPLES: u32 = 10;

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    renderer.render("./output.png");
}