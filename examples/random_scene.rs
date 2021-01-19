extern crate raytracer;

use pbr::ProgressBar;

use raytracer::renderer::Renderer;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const N_SAMPLES: u32 = 10;

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let mut progress_bar = ProgressBar::new(100);

    renderer.render("./output.png", move |current_progress| {
        progress_bar.set((current_progress * 100.0) as u64);
    });
}