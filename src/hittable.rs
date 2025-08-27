use crate::interval::Interval;
use crate::vec3::{dot, Point3, Vec3};
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new() -> Self {
    HitRecord { 
      p: Point3::new(), 
      normal: Vec3::new(),
      t: 0.0,
      front_face: false,
    }
  }

  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
    self.front_face = dot(ray.direction(), outward_normal) < 0.0;
    self.normal = if self.front_face {
      *outward_normal
    } else {
      -*outward_normal
    };
  }
}


pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}