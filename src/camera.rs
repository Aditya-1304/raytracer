use crate::material::Material;
use crate::vec3::{random_on_hemisphere, random_unit_vector, unit_vector, Point3, Vec3};
use crate::ray::Ray;
use crate::color::{Color, write_color};
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::rtweekend::{INFINITY, random_float};
use std::io::{self, Write};

pub struct Camera {
  pub aspect_ratio: f64,
  pub image_width: i32,
  pub samples_per_pixel: i32,
  pub max_depth: i32,

  image_height: i32,
  pixel_samples_scale: f64,
  center: Point3,
  pixel00_loc: Point3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
}

impl Camera {
  pub fn new() -> Self {
    Camera {
      aspect_ratio: 1.0,
      image_width: 100,
      samples_per_pixel: 10,
      max_depth: 10,
      image_height: 0,
      pixel_samples_scale: 0.0,
      center: Point3::new(),
      pixel00_loc: Point3::new(),
      pixel_delta_u: Vec3::new(),
      pixel_delta_v: Vec3::new(),
    }
  }

  pub fn render<W: Write>(&mut self, world: &dyn Hittable, writer: &mut W) -> io::Result<()> {
    self.initialize();

    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", self.image_width, self.image_height)?;
    writeln!(writer, "255")?;

    for j in 0..self.image_height {
      eprint!("\rScanlines remaining: {} ", self.image_height - j);
      io::stderr().flush()?;

      for i in 0..self.image_width {

        let mut pixel_color = Color::from_values(0.0, 0.0, 0.0);
        for _sample in 0..self.samples_per_pixel {
          let ray = self.get_ray(i, j);
          pixel_color = pixel_color + self.ray_color(&ray, self.max_depth, world);
        }
          write_color(writer, &(self.pixel_samples_scale * pixel_color))?;
      }
    }

    eprintln!("\rDone.                ");
    Ok(())
  }

  fn initialize(&mut self) {
    self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
    self.image_height = if self.image_height < 1 {1} else { self.image_height };

    self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

    self.center = Point3::from_values(0.0, 0.0, 0.0);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

    let viewport_u = Vec3::from_values(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from_values(0.0, -viewport_height, 0.0);

    self.pixel_delta_u = viewport_u / self.image_width as f64;
    self.pixel_delta_v = viewport_v / self.image_height as f64;

    let viewport_upper_left = self.center
      - Vec3::from_values(0.0, 0.0, focal_length)
      - viewport_u / 2.0
      - viewport_v / 2.0;
    self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
  }

  fn get_ray(&self, i: i32, j: i32) -> Ray {
    let offset = self.sample_square();
    let pixel_sample = self.pixel00_loc
      + ((i as f64 + offset.x()) * self.pixel_delta_u)
      + ((j as f64 + offset.y()) * self.pixel_delta_v);

    let ray_origin = self.center;
    let ray_direction = pixel_sample - ray_origin;

    Ray::from_origin_direction(ray_origin, ray_direction)
  }

  fn sample_square(&self) -> Vec3 {
    Vec3::from_values(random_float() - 0.5, random_float() - 0.5, 0.0)
  }

  fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
    if depth <= 0{
      return Color::from_values(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new();

    if world.hit(ray, Interval::from_range(0.0001, INFINITY), &mut rec) {
      // let direction = random_on_hemisphere(&rec.normal);
      let mut scattered = Ray::new();
      let mut attenuation = Color::new();
      // let direction = rec.normal + random_unit_vector();
      // return 0.9 * self.ray_color(&Ray::from_origin_direction(rec.p, direction),depth - 1, world);

      if let Some(material) = &rec.mat {
        if material.scatter(ray, &rec, &mut attenuation, &mut scattered) {
          return attenuation * self.ray_color(&scattered, depth - 1, world);
        }
      }
      return Color::from_values(0.0, 0.0, 0.0)
    }

    let unit_direction = unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::from_values(1.0, 1.0, 1.0) + a * Color::from_values(0.5, 0.7, 1.0)
  }
}

impl Default for Camera {
  fn default() -> Self {
    Self::new()
  }
}

