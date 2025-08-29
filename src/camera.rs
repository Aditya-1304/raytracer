// use crate::material::Material;
use crate::vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3};
use crate::ray::Ray;
use crate::color::{Color, write_color};
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::rtweekend::{degrees_to_radians, random_float, INFINITY};
use std::io::{self, Write};

pub struct Camera {
  pub aspect_ratio: f64,
  pub image_width: i32,
  pub samples_per_pixel: i32,
  pub max_depth: i32,
  pub vfov: f64,
  pub lookfrom: Point3,
  pub lookat: Point3,
  pub vup: Vec3,
  pub defocus_angle: f64,
  pub focus_dist: f64,

  image_height: i32,
  pixel_samples_scale: f64,
  center: Point3,
  pixel00_loc: Point3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  defocus_disk_u : Vec3,
  defocus_disk_v: Vec3,
}

impl Camera {
  pub fn new() -> Self {
    Camera {
      aspect_ratio: 1.0,
      image_width: 100,
      samples_per_pixel: 10,
      max_depth: 10,
      vfov: 90.0,
      lookfrom: Point3::from_values(0.0, 0.0, 0.0),
      lookat: Point3::from_values(0.0, 0.0, -1.0),
      vup: Vec3::from_values(0.0, 1.0, 0.0),
      defocus_angle: 0.0,
      focus_dist: 10.0,


      image_height: 0,
      pixel_samples_scale: 0.0,
      center: Point3::new(),
      pixel00_loc: Point3::new(),
      pixel_delta_u: Vec3::new(),
      pixel_delta_v: Vec3::new(),
      u: Vec3::new(),
      v: Vec3::new(),
      w: Vec3::new(),
      defocus_disk_u: Vec3::new(),
      defocus_disk_v: Vec3::new(),
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

    self.center = self.lookfrom;

    // let focal_length = (self.lookfrom - self.lookat).length();
    let theta = degrees_to_radians(self.vfov);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h * self.focus_dist;
    let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

    self.w = unit_vector(&(self.lookfrom - self.lookat));
    self.u = unit_vector(&cross(&self.vup, &self.w));
    self.v = cross(&self.w, &self.u);

    let viewport_u = viewport_width * self.u;
    let viewport_v = viewport_height * (-self.v);

    self.pixel_delta_u = viewport_u / self.image_width as f64;
    self.pixel_delta_v = viewport_v / self.image_height as f64;

    let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
    self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

    let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
    self.defocus_disk_u = self.u * defocus_radius;
    self.defocus_disk_v = self.v * defocus_radius;
  }

  fn get_ray(&self, i: i32, j: i32) -> Ray {
    let offset = self.sample_square();
    let pixel_sample = self.pixel00_loc
      + ((i as f64 + offset.x()) * self.pixel_delta_u)
      + ((j as f64 + offset.y()) * self.pixel_delta_v);

    let ray_origin = if self.defocus_angle <= 0.0 {
      self.center
    } else {
      self.defocus_disk_sample()
    };
    let ray_direction = pixel_sample - ray_origin;

    Ray::from_origin_direction(ray_origin, ray_direction)
  }

  fn sample_square(&self) -> Vec3 {
    Vec3::from_values(random_float() - 0.5, random_float() - 0.5, 0.0)
  }

  fn defocus_disk_sample(&self) -> Point3 {
    let p = random_in_unit_disk();
    self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
  }

  fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
    if depth <= 0{
      return Color::from_values(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new();

    if world.hit(ray, Interval::from_range(0.001, INFINITY), &mut rec) {
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

