extern crate raytracer;

use raytracer::rendering::render;
use raytracer::rendering::skyboxes::GradientSkybox;
use raytracer::rendering::Camera;
use raytracer::rendering::RenderParams;
use raytracer::structures::{Color, Vec3, Point3, Quaternion, Transform};
use raytracer::hittables::{BVHNode, HittableList, Sphere, MovingInstance};
use raytracer::materials::{Metal, Lambertian, Dieletric};

use std::sync::Arc;
use std::time::{Instant};

use rand::prelude::{thread_rng, Rng};

use pbr::ProgressBar;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const NUM_SAMPLES: u32 = 500;
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
    let camera = Arc::new(Camera::new(Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), (20.0 as f64).to_radians(), aspect_ratio, 0.1, 10.0, 0.0, 1.0));
    let skybox = Arc::new(GradientSkybox::new(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), Vec3::new(0.0, 1.0, 0.0)));
    let world = Arc::new(BVHNode::new(&build_scene(), 0.0, 1.0));
    
    progress_bar.set(0);

    let start = Instant::now();

    render(world, skybox, camera, &params, move |sampled, _| {
        progress_bar.set(sampled as u64);
    }).save("./output.png");

    let duration = start.elapsed();

    println!("Time Elapsed: {:?}", duration);
}

fn build_scene() -> HittableList {
    let mut rng = thread_rng();

    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    world.add(ground);

    for i in -11..=11 {
        for j in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let x = (i as f64) + 0.9 * rng.gen::<f64>();
            let z = (j as f64) + 0.9 * rng.gen::<f64>();
            let center = Point3::new(x, 0.2, z);

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let material = Arc::new(Lambertian::new(albedo));
                    
                    let position_0 = center;
                    let position_1 = center + Vec3::up() * rng.gen_range(0.0..=0.5);
                    let rotation = Quaternion::default();
                    let scale = Vec3::new(1.0, 1.0, 1.0);

                    let transform_0 = Transform::new(position_0, rotation, scale);
                    let transform_1 = Transform::new(position_1, rotation, scale);
                    
                    let sphere = Arc::new(Sphere::new(Vec3::zero(), 0.2, material));
                    world.add(Arc::new(MovingInstance::new(sphere, transform_0, transform_1, 0.0, 1.0)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0) * Color::random(0.5, 1.0);
                    let fuzz: f64 = rng.gen_range(0.0..=0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Arc::new(Dieletric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dieletric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));
    
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.7, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}