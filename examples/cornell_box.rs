extern crate raytracer;

use raytracer::rendering::{render, Camera, RenderParams};
use raytracer::skyboxes::SolidColorSkybox;
use raytracer::textures::SolidColor;
use raytracer::structures::{Color, Vec3, Point3};
use raytracer::hittables::{BVHNode, HittableList, XYRect, XZRect, YZRect};
use raytracer::materials::{Lambertian, DiffuseLight};

use std::sync::Arc;
use std::time::Instant;

use pbr::ProgressBar;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const NUM_SAMPLES: u32 = 200;
const MAX_RAY_DEPTH: u32 = 50;

fn main() {
    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let mut progress_bar = ProgressBar::new(NUM_SAMPLES as u64);
    
    let params = RenderParams {
        image_width: WIDTH,
        image_height: HEIGHT,
        num_samples: NUM_SAMPLES,
        max_ray_depth: MAX_RAY_DEPTH
    };
    let camera = Arc::new(Camera::new(Point3::new(278.0, 278.0, -800.0), Point3::new(278.0, 278.0, 0.0), Vec3::new(0.0, 1.0, 0.0), (40.0 as f64).to_radians(), aspect_ratio, 0.0, 10.0, 0.0, 1.0));
    let skybox = Arc::new(SolidColorSkybox::new(Color::new(0.0, 0.0, 0.0)));
    let world = Arc::new(BVHNode::new(&build_scene(), 0.0, 1.0));
    
    progress_bar.set(0);

    let start = Instant::now();

    render(world, skybox, camera, &params, move |sampled, _| {
        progress_bar.set(sampled as u64);
    }).save("./cornell_box.png");

    let duration = start.elapsed();

    println!("Time Elapsed: {:?}", duration);
}

fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let red_texture = Arc::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let green_texture = Arc::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let white_texture = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let light_texture = Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)));

    let red_material = Arc::new(Lambertian::new(red_texture));
    let green_material = Arc::new(Lambertian::new(green_texture));
    let white_material = Arc::new(Lambertian::new(white_texture));
    let light_material = Arc::new(DiffuseLight::new(light_texture));

    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green_material.clone())));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red_material.clone())));
    world.add(Arc::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light_material.clone())));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white_material.clone())));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material.clone())));
    world.add(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material.clone())));

    world
}