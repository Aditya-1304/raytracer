use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::bvh::AABB;
use std::sync::Arc;

pub struct HittableList {
  pub objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
  pub fn _new() -> Self {
    HittableList { objects: Vec::new() }
  }

  pub fn _from_object(object: Arc<dyn Hittable>) -> Self {
    let mut list = HittableList::_new();
    list._add(object);
    list
  }

  pub fn _clear(&mut self) {
    self.objects.clear();
  }

  pub fn _add(&mut self, object: Arc<dyn Hittable>) {
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
      let mut temp_rec = HitRecord::new();
      let mut hit_anything = false;
      let mut closest_so_far = ray_t.max;

      for object in &self.objects {
        if object.hit(ray, Interval::from_range(ray_t.min, closest_so_far), &mut temp_rec) {
          hit_anything = true;
          closest_so_far = temp_rec.t;
          *rec = temp_rec.clone();
        }
      }
      
      hit_anything
  }

  fn bounding_box(&self) -> AABB {
    if self.objects.is_empty() {
      return AABB::new(); 
    }

    let mut output_box = self.objects[0].bounding_box();
    
    for object in &self.objects[1..] {
      output_box = AABB::from_boxes(&output_box, &object.bounding_box());
    }
    
    output_box
  }
}