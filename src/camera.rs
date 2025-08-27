use crate::vec3::{Vec3, Point3, unit_vector};
use crate::ray::Ray;
use crate::color::{Color, write_color};
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::rtweekend::INFINITY;
use std::io::{self, Write};

pub struct Camera {
  pub aspect_ratio: f64,
  pub image_width: i32,

  image_height: i32,
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
      image_height: 0,
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

    for j in 0..self.image_width {
      eprint!("\rScanlines remaining: {} ", self.image_height - j);
      io::stderr().flush()?;

      for i in 0..self.image_width {
        let pixel_center = self.pixel00_loc
          + (i as f64 * self.pixel_delta_u)
          + (j as f64 * self.pixel_delta_v);
          let ray_direction = pixel_center - self.center;
          let ray = Ray::from_origin_direction(self.center, ray_direction);

          let pixel_color = self.ray_color(&ray, world);
          write_color(writer, &pixel_color)?;
      }
    }

    eprintln!("\rDone.                ");
    Ok(())
  }

  fn initialize(&mut self) {
    self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
    self.image_height = if self.image_height < 1 {1} else { self.image_height };

    self.center = Point3::from_values(0.0, 0.0, 0.0);

    let focal_lenght = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

    let viewport_u = Vec3::from_values(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from_values(0.0, -viewport_height, 0.0);

    self.pixel_delta_u = viewport_u / self.image_width as f64;
    self.pixel_delta_v = viewport_v / self.image_height as f64;

    let viewport_upper_left = self.center
      - Vec3::from_values(0.0, 0.0, focal_lenght)
      - viewport_u / 2.0
      - viewport_v / 2.0;
    self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
  }

  fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(ray, Interval::from_range(0.0, INFINITY), &mut rec) {
      return 0.5 * (rec.normal + Color::from_values(1.0, 1.0, 1.0))
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