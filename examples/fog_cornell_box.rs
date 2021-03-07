extern crate raytracer;

use raytracer::rendering::{render, Camera, RenderParams};
use raytracer::skyboxes::SolidColorSkybox;
use raytracer::textures::SolidColor;
use raytracer::structures::{Color, Vec3, Point3, Transform, Quaternion};
use raytracer::hittables::{BVHNode, HittableList, XYRect, XZRect, YZRect, AABox, Instance, ConstantMedium};
use raytracer::materials::{Lambertian, DiffuseLight};

use std::sync::Arc;
use std::time::Instant;

use pbr::ProgressBar;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const NUM_SAMPLES: u32 = 1000;
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
    }).save("./fog_cornell_box.png");

    let duration = start.elapsed();

    println!("Time Elapsed: {:?}", duration);
}

fn build_scene() -> HittableList {
    let mut world = HittableList::new();

    let red_texture = Arc::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let green_texture = Arc::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let white_texture = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let light_texture = Arc::new(SolidColor::new(Color::new(7.0, 7.0, 7.0)));
    let fog_0_texture = Arc::new(SolidColor::new(Color::new(0.0, 0.0, 0.0)));
    let fog_1_texture = Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0)));
    
    let red_material = Arc::new(Lambertian::new(red_texture));
    let green_material = Arc::new(Lambertian::new(green_texture));
    let white_material = Arc::new(Lambertian::new(white_texture));
    let light_material = Arc::new(DiffuseLight::new(light_texture));

    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green_material.clone())));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red_material.clone())));
    world.add(Arc::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light_material.clone())));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white_material.clone())));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material.clone())));
    world.add(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material.clone())));

    let box_0 = Arc::new(AABox::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white_material.clone()));
    let box_1 = Arc::new(AABox::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white_material.clone()));

    let transform_0 = Transform::new(Vec3::new(265.0, 0.0, 295.0), Quaternion::from_axis_angle(Vec3::up(), f64::to_radians(15.0)), Vec3::new(1.0, 1.0, 1.0));
    let transform_1 = Transform::new(Vec3::new(130.0, 0.0, 65.0), Quaternion::from_axis_angle(Vec3::up(), f64::to_radians(-18.0)), Vec3::new(1.0, 1.0, 1.0));

    let fog_0 = Arc::new(ConstantMedium::new(box_0, 0.01, fog_0_texture));
    let fog_1 = Arc::new(ConstantMedium::new(box_1, 0.01, fog_1_texture));

    world.add(Arc::new(Instance::new(fog_0, transform_0)));
    world.add(Arc::new(Instance::new(fog_1, transform_1)));

    world
}