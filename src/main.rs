use std::env;
use std::fs::File;
use std::io::{self};
use std::sync::Arc;
use vec3::Point3;
use sphere::Sphere;
use camera::Camera;
use bvh::BVHNode;  

use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::rtweekend::{random_float, random_float_range};
use crate::vec3::Vec3;

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
mod bvh;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "image.ppm"
    };

    let mut objects: Vec<Arc<dyn crate::hittable::Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.5, 0.5, 0.5)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point3::from_values(
                a as f64 + 0.9 * random_float(),
                0.2,
                b as f64 + 0.9 * random_float()
            );

            if (center - Point3::from_values(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::from_values(0.4, 0.2, 0.1)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::from_values(0.7, 0.6, 0.5), 0.0));
    objects.push(Arc::new(Sphere::new(Point3::from_values(4.0, 1.0, 0.0), 1.0, material3)));


    let world = BVHNode::new(objects);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    
    
    camera.image_width = 800;       
    camera.samples_per_pixel = 100; 
    camera.max_depth = 30;          

    camera.vfov = 20.0;
    camera.lookfrom = Point3::from_values(13.0, 2.0, 3.0);
    camera.lookat = Point3::from_values(0.0, 1.0, 0.0);
    camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    
    println!("Image saved to {}", filename);
    Ok(())
}