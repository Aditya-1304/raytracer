use crate::vec3::{Vec3, Point3, dot};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
  center: Point3,
  radius: f64,
  material: Arc<dyn Material>
}

impl Sphere {
  pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
    Sphere { 
      center, 
      radius: radius.max(0.0),
      material : material,
    }
  }

  pub fn bounding_box(&self) -> crate::bvh::AABB {
    let rvec = Vec3::from_values(self.radius, self.radius, self.radius);
    crate::bvh::AABB::from_points(
      self.center - rvec,
      self.center + rvec
    )
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
      let oc = self.center - *ray.origin();
      let a = ray.direction().length_squared();
      let h = dot(ray.direction(), &oc);
      let c = oc.length_squared() - self.radius * self.radius;

      let discriminant = h * h - a * c;
      if discriminant < 0.0 {
        return false;
      }

      let sqrt_discriminant = discriminant.sqrt();

      let mut root = (h - sqrt_discriminant) / a;
      if !ray_t.surrounds(root) {
        root = (h + sqrt_discriminant) / a;
        if !ray_t.surrounds(root) {
          return false;
        }
      }

      rec.t = root;
      rec.p = ray.at(rec.t);
      let outward_normal = (rec.p - self.center) /self.radius;
      rec.set_face_normal(ray, &outward_normal);
      rec.mat = Some(self.material.clone());

      true
  }

  fn bounding_box(&self) -> crate::bvh::AABB {
    self.bounding_box()
  }
}

impl std::fmt::Debug for Sphere {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Sphere")
        .field("center", &self.center)
        .field("radius", &self.radius)
        .field("material", &"<Material>")
        .finish()
  }
}