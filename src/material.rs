use crate::vec3::{random_unit_vector, reflect, Vec3};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

pub trait Material {
    fn scatter(
      &self,
      r_in: &Ray,
      rec: &HitRecord,
      attenuation: &mut Color,
      scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
  albedo: Color,
}

impl Lambertian {
  pub fn new(albedo: Color) -> Self {
    Lambertian { albedo }
  } 
}

impl Material for Lambertian {
  fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
      ) -> bool {
      let mut scatter_direction = rec.normal + random_unit_vector();

      if scatter_direction.near_zero() {
        scatter_direction = rec.normal;
      }

      *scattered = Ray::from_origin_direction(rec.p, scatter_direction);
      *attenuation = self.albedo;
      true
  }
}

pub struct Metal {
  albedo: Color,
}

impl Metal {
  pub fn new(albedo: Color) -> Self {
    Metal { albedo }
  }
}

impl Material for Metal {
  fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
      ) -> bool {
      let reflected = reflect(r_in.direction(), &rec.normal);
      *scattered = Ray::from_origin_direction(rec.p, reflected);
      *attenuation = self.albedo;
      true
  }
}