extern crate raytracer;

use raytracer::rendering::{render, Camera, RenderParams};
use raytracer::skyboxes::SolidColorSkybox;
use raytracer::textures::{Noise, SolidColor};
use raytracer::structures::{Color, Vec3, Point3};
use raytracer::hittables::{BVHNode, HittableList, Sphere, XYRect};
use raytracer::materials::{Lambertian, DiffuseLight};

use std::sync::Arc;
use std::time::Instant;

use pbr::ProgressBar;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const NUM_SAMPLES: u32 = 100;
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
    let camera = Arc::new(Camera::new(Point3::new(26.0, 3.0, 6.0), Point3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 1.0, 0.0), (20.0 as f64).to_radians(), aspect_ratio, 0.0, 10.0, 0.0, 1.0));
    let skybox = Arc::new(SolidColorSkybox::new(Color::new(0.0, 0.0, 0.0)));
    let world = Arc::new(BVHNode::new(&build_scene(), 0.0, 1.0));
    
    progress_bar.set(0);

    let start = Instant::now();

    render(world, skybox, camera, &params, move |sampled, _| {
        progress_bar.set(sampled as u64);
    }).save("./area_light.png");

    let duration = start.elapsed();

    println!("Time Elapsed: {:?}", duration);
}

fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let noise_texture = Arc::new(Noise::new(4.0));
    let material = Arc::new(Lambertian::new(noise_texture));

    let sphere_0 = Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material.clone()));
    world.add(sphere_0);
    let sphere_1 = Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, material.clone()));
    world.add(sphere_1);


    let light_texture = Arc::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)));
    let light_material = Arc::new(DiffuseLight::new(light_texture));

    let light = Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, light_material.clone()));
    world.add(light);

    world
}