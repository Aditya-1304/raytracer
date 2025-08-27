use crate::vec3::{Vec3, Point3, dot};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

#[derive(Debug,Clone)]
pub struct Sphere {
  center: Point3,
  radius: f64,
}

impl Sphere {
  pub fn new(center: Point3, radius: f64) -> Self {
    Sphere { center, radius: radius.max(0.0) }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
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
      if root <= ray_tmin || ray_tmax <= root {
        root = (h + sqrt_discriminant) / a;
        if root <= ray_tmin || ray_tmax <= root {
          return false;
        }
      }

      rec.t = root;
      rec.p = ray.at(rec.t);
      let outward_normal = (rec.p - self.center) /self.radius;
      rec.set_face_normal(ray, &outward_normal);

      true
  }
}