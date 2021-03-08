extern crate raytracer;

use raytracer::rendering::{render, Camera, RenderParams};
use raytracer::skyboxes::SolidColorSkybox;
use raytracer::textures::{SolidColor, ImageTexture, Noise};
use raytracer::structures::{Color, Vec3, Point3, Transform, Quaternion};
use raytracer::hittables::{BVHNode, HittableList, Sphere, XZRect, AABox, Instance, MovingInstance, ConstantMedium};
use raytracer::materials::{Lambertian, Dieletric, Metal, DiffuseLight};

use std::sync::Arc;
use std::time::Instant;

use rand::{thread_rng, Rng};
use pbr::ProgressBar;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const NUM_SAMPLES: u32 = 10000;
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
    let camera = Arc::new(Camera::new(Point3::new(478.0, 278.0, -600.0), Point3::new(278.0, 278.0, 0.0), Vec3::new(0.0, 1.0, 0.0), (40.0 as f64).to_radians(), aspect_ratio, 0.0, 10.0, 0.0, 1.0));
    let skybox = Arc::new(SolidColorSkybox::new(Color::new(0.0, 0.0, 0.0)));
    let world = Arc::new(BVHNode::new(&build_scene(), 0.0, 1.0));
    
    progress_bar.set(0);

    let start = Instant::now();

    render(world, skybox, camera, &params, move |sampled, _| {
        progress_bar.set(sampled as u64);
    }).save("./book_two.png");

    let duration = start.elapsed();

    println!("Time Elapsed: {:?}", duration);
}

fn build_scene() -> HittableList {
    let mut rng = thread_rng();
    let mut world = HittableList::new();

    let ground_texture = Arc::new(SolidColor::new(Color::new(0.48, 0.83, 0.53)));
    let ground_material = Arc::new(Lambertian::new(ground_texture.clone()));
    let mut ground = HittableList::new();

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let box_width = 100.0;
            let x0 = -1000.0 + (i as f64) * box_width;
            let y0 = 0.0;
            let z0 = -1000.0 + (j as f64) * box_width;
            let x1 = x0 + box_width;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + box_width;

            ground.add(Arc::new(AABox::new(Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), ground_material.clone())));
        }
    }

    let ground = Arc::new(BVHNode::new(&ground, 0.0, 1.0));
    world.add(ground);

    let light_texture = Arc::new(SolidColor::new(Color::new(14.0, 14.0, 14.0)));
    let light_material = Arc::new(DiffuseLight::new(light_texture));
    let light = Arc::new(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light_material));
    world.add(light);

    let transform_0 = Transform::new(Vec3::new(400.0, 400.0, 200.0), Quaternion::default(), Vec3::new(1.0, 1.0, 1.0));
    let transform_1 = Transform::new(Vec3::new(430.0, 400.0, 200.0), Quaternion::default(), Vec3::new(1.0, 1.0, 1.0));
    let moving_sphere_texture = Arc::new(SolidColor::new(Color::new(0.7, 0.3, 0.1)));
    let moving_sphere_material = Arc::new(Lambertian::new(moving_sphere_texture));
    let moving_sphere = Arc::new(Sphere::new(Point3::zero(), 50.0, moving_sphere_material));
    let moving_sphere = Arc::new(MovingInstance::new(moving_sphere, transform_0, transform_1, 0.0, 1.0));
    world.add(moving_sphere);

    let glass_material = Arc::new(Dieletric::new(1.5));
    let glass_sphere = Arc::new(Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, glass_material.clone()));
    world.add(glass_sphere);

    let metal_texture = Arc::new(SolidColor::new(Color::new(0.8, 0.8, 0.9)));
    let metal_material = Arc::new(Metal::new(metal_texture, 1.0));
    let metal_sphere = Arc::new(Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, metal_material));
    world.add(metal_sphere);

    let sss_boundary = Arc::new(Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, glass_material.clone()));
    let sss_texture = Arc::new(SolidColor::new(Color::new(0.2, 0.4, 0.9)));
    let sss_sphere = Arc::new(ConstantMedium::new(sss_boundary.clone(), 0.2, sss_texture));
    world.add(sss_boundary);
    world.add(sss_sphere);

    let fog_boundary = Arc::new(Sphere::new(Point3::zero(), 5000.0, glass_material.clone()));
    let fog_texture = Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0)));
    let fog = Arc::new(ConstantMedium::new(fog_boundary, 0.0001, fog_texture));
    world.add(fog);

    let earth_texture = Arc::new(ImageTexture::read("./resources/earthmap.jpg"));
    let earth_material = Arc::new(Lambertian::new(earth_texture));
    let earth = Arc::new(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, earth_material));
    world.add(earth);

    let perlin_texture = Arc::new(Noise::new(0.1));
    let perlin_material = Arc::new(Lambertian::new(perlin_texture));
    let perlin_sphere = Arc::new(Sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, perlin_material));
    world.add(perlin_sphere);

    let sphere_box_texture = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let sphere_box_material = Arc::new(Lambertian::new(sphere_box_texture));
    let mut sphere_box = HittableList::new();
    
    let num_spheres = 1000;
    for _ in 0..num_spheres {
        let sphere = Arc::new(Sphere::new(Point3::random_between(0.0, 165.0), 10.0, sphere_box_material.clone()));
        sphere_box.add(sphere);
    }
    
    let sphere_box_transform = Transform::new(Vec3::new(-100.0, 270.0, 395.0), Quaternion::from_axis_angle(Vec3::up(), f64::to_radians(15.0)), Vec3::new(1.0, 1.0, 1.0));
    let sphere_box = Arc::new(BVHNode::new(&sphere_box, 0.0, 1.0));
    let sphere_box = Arc::new(Instance::new(sphere_box, sphere_box_transform));
    world.add(sphere_box);

    world
}