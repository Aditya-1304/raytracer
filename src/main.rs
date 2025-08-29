use std::env;
use std::fs::File;
use std::io::{self};
use std::sync::Arc;
use vec3::{ Point3};
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;

use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::rtweekend::{random_float, random_float_range, PI};
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


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "image.ppm"
    };

    // let material_ground = Arc::new(Lambertian::new(Color::from_values(0.8, 0.8, 0.0)));
    // let material_center = Arc::new(Lambertian::new(Color::from_values(0.1, 0.2, 0.5)));
    // // let material_left = Arc::new(Metal::new(Color::from_values(0.8, 0.8, 0.8),0.3));
    // let material_left = Arc::new(Dielectric::new(1.50));
    // let material_bubble = Arc::new(Dielectric::new(1.00/ 1.5));
    // let material_right = Arc::new(Metal::new(Color::from_values(0.8, 0.6, 0.2), 1.0));

    // let mut world = HittableList::new();
    // world.add(Arc::new(Sphere::new(Point3::from_values(0.0, -100.5, -1.0), 100.0, material_ground)));
    // world.add(Arc::new(Sphere::new(Point3::from_values(0.0, 0.0, -1.2), 0.5, material_center)));
    // world.add(Arc::new(Sphere::new(Point3::from_values(-1.0, 0.0, -1.0), 0.5, material_left)));
    // world.add(Arc::new(Sphere::new(Point3::from_values(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    // world.add(Arc::new(Sphere::new(Point3::from_values(1.0, 0.0, -1.0), 0.5, material_right)));

    // let mut camera = Camera::new();
    // camera.aspect_ratio = 16.0 / 9.0;
    // camera.image_width = 400;
    // camera.samples_per_pixel = 100;
    // camera.max_depth = 50;


    // camera.vfov = 50.0;
    // camera.lookfrom = Point3::from_values(-2.0, 2.0, 1.0);
    // camera.lookat = Point3::from_values(0.0, 0.0, -1.0);
    // camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    // camera.defocus_angle = 10.0;
    // camera.focus_dist = 3.4;

    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));

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
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::from_values(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::from_values(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::from_values(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::from_values(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::from_values(4.0, 1.0, 0.0),1.0, material3)));


    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::from_values(13.0, 2.0, 3.0);
    camera.lookat = Point3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    
    println!("Image saved to {}", filename);
    Ok(())
}
