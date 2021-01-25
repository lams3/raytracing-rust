extern crate raytracer;

use std::rc::Rc;

use rand::prelude::{thread_rng, Rng};

use pbr::ProgressBar;

use raytracer::structures::{Color, Vec3, Point3};
use raytracer::hittables::{HittableList, Sphere};
use raytracer::renderer::{Renderer, Camera};
use raytracer::renderer::skybox::GradientSkybox;
use raytracer::materials::{Metal, Lambertian, Dieletric};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const N_SAMPLES: u32 = 500;
const MAX_DEPTH: u32 = 50;

fn main() {
    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let mut renderer = Renderer::new(WIDTH, HEIGHT, N_SAMPLES, MAX_DEPTH);
    let mut progress_bar = ProgressBar::new(100);

    let camera = Camera::new(Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), (20.0 as f64).to_radians(), aspect_ratio, 0.1, 10.0);

    let skybox = GradientSkybox::new(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), Vec3::new(0.0, 1.0, 0.0));
    let world = build_scene();

    renderer.render("./output.png", &world, &skybox, &camera, move |current_progress| {
        progress_bar.set((current_progress * 100.0) as u64);
    });
}

fn build_scene() -> HittableList {
    let mut rng = thread_rng();

    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

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
                    let material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0) * Color::random(0.5, 1.0);
                    let fuzz: f64 = rng.gen_range(0.0..=0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Rc::new(Dieletric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dieletric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));
    
    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.7, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}