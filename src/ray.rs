use crate::vec3::{Vec3, Point3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
  origin : Point3,
  direction: Vec3,
}

impl Ray {
  pub fn new() -> Self {
    Ray { origin: Point3::new(), direction: Vec3::new() }
  }

  pub fn from_origin_direction(origin: Point3, direction: Vec3) -> Self {
    Ray { origin: origin, direction: direction }
  }

  pub fn origin(&self) -> &Point3 {
    &self.origin
  }

  pub fn direction(&self) -> &Vec3 {
    &self.direction
  }

  pub fn at(&self, t: f64) -> Point3 {
    self.origin + t * self.direction
  }
}

impl Default for Ray {
  fn default() -> Self {
      Self::new()
  }
}
