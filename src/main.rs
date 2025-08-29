use std::env;
use std::fs::File;
use std::io::{self};
use std::sync::Arc;
use vec3::{ Point3};
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;

use crate::color::Color;
use crate::material::{Lambertian, Metal};


mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod interval;
mod camera;
mod material;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "image.ppm"
    };

    let material_ground = Arc::new(Lambertian::new(Color::from_values(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::from_values(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::from_values(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Color::from_values(0.8, 0.6, 0.2)));

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::from_values(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(Point3::from_values(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(Point3::from_values(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(Point3::from_values(1.0, 0.0, -1.0), 0.5, material_right)));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    
    println!("Image saved to {}", filename);
    Ok(())
}
