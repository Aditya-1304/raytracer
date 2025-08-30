use crate::rtweekend::random_float;
use crate::vec3::{dot, random_unit_vector, reflect, refract, unit_vector};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

pub trait Material: Send + Sync {
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
        
        let albedo_max = self.albedo.x().max(self.albedo.y()).max(self.albedo.z());
        if random_float() > albedo_max {
            return false;
        }

        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::from_origin_direction(rec.p, scatter_direction);
        *attenuation = self.albedo / albedo_max; 
        true
    }
}

pub struct Metal {
  albedo: Color,
  fuzz: f64
}

impl Metal {
  pub fn new(albedo: Color, fuzz: f64) -> Self {
    Metal { 
      albedo,
      fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }  
    }
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
      let reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
      *scattered = Ray::from_origin_direction(rec.p, reflected);
      *attenuation = self.albedo;
      
      dot(scattered.direction(), &rec.normal) > 0.0
  }
}

pub struct Dielectric {
  refraction_index: f64,
}

impl Dielectric {
  pub fn new(refraction_index: f64) -> Self {
    Dielectric { refraction_index }
  }
  fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
  }
}

impl Material for Dielectric {
  fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
      ) -> bool {
      *attenuation = Color::from_values(1.0, 1.0, 1.0);
      let ri = if rec.front_face {
        1.0 / self.refraction_index
      } else {
        self.refraction_index
      };

      let unit_direction = unit_vector(r_in.direction());
      let cos_theta = dot(&(-unit_direction), &rec.normal).min(1.0);
      let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

      let cannot_refract = ri * sin_theta > 1.0;
      let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_float() {
        reflect(&unit_direction, &rec.normal)
      } else {
        refract(&unit_direction, &rec.normal, ri)
      };
      
      *scattered = Ray::from_origin_direction(rec.p, direction);
      true
  }
}