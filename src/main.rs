use std::env;
use std::fs::File;
use std::io::{self};
use std::sync::Arc;
use vec3::{ Point3};
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;


mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod interval;
mod camera;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "image.ppm"
    };

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::from_values(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::from_values(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    
    println!("Image saved to {}", filename);
    Ok(())
}
