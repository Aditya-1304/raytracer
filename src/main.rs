use std::env;
use std::fs::File;
use std::io::{self, Write};
use color::{Color, write_color};
use ray::Ray;
use vec3::{Vec3, Point3,dot, unit_vector};

mod vec3;
mod color;
mod ray;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = *center - *ray.origin();
    let a = dot(ray.direction(), ray.direction());
    let b = -2.0 * dot(ray.direction(), &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(ray: &Ray) -> Color {

    if hit_sphere(&Point3::from_values(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::from_values(1.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::from_values(1.0, 1.0, 1.0) + a * Color::from_values(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "image.ppm"
    };

    let aspect_ratio = 16.0/9.0;
    let image_width  = 400;
    

    let mut image_height  = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };
    


    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64/ image_height as f64);
    let camera_center = Point3::from_values(0.0,0.0,0.0);

    let viewport_u = Vec3::from_values(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from_values(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center 
        - Vec3::from_values(0.0 ,0.0, focal_length) 
        - viewport_u/2.0 
        - viewport_v/2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut file = File::create(filename)?;

    writeln!(file, "P3")?;
    writeln!(file,"{} {}", image_width, image_height)?;
    writeln!(file,"255")?;

   for j in 0..image_height {
    eprint!("\rScanlines remaining: {}", image_height - j);
    io::stderr().flush()?;


        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::from_origin_direction(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut file, &pixel_color)?;
        }
    }
    eprintln!("\rDone.                 ");
    println!("Image saved to {}", filename);
    Ok(())
}
