use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use std::sync::Arc;

pub struct HittableList {
  pub objects: Vec<Arc<dyn Hittable>>
}
impl HittableList {
  pub fn new() -> Self {
    HittableList { objects: Vec::new() }
  }

  pub fn from_object(object: Arc<dyn Hittable>) -> Self {
    let mut list = HittableList::new();
    list.add(object);
    list
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }

  pub fn add(&mut self, object: Arc<dyn Hittable>) {
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
      let mut temp_rec = HitRecord::new();
      let mut hit_anything = false;
      let mut closest_so_far = ray_tmax;

      for object in &self.objects {
        if object.hit(ray, ray_tmin, closest_so_far, &mut temp_rec) {
          hit_anything = true;
          closest_so_far = temp_rec.t;
          *rec = temp_rec.clone();
        }
      }
      
      hit_anything
  }
}