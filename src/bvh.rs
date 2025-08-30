use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::rtweekend::random_float;
use crate::vec3::Point3;
use std::sync::Arc;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct AABB { 
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new() -> Self {
        AABB {
            min: Point3::from_values(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Point3::from_values(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        AABB {
            min: Point3::from_values(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z())),
            max: Point3::from_values(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z())),
        }
    }

    pub fn from_boxes(box0: &AABB, box1: &AABB) -> AABB {
        AABB::from_points(
            Point3::from_values(
                box0.min.x().min(box1.min.x()),
                box0.min.y().min(box1.min.y()),
                box0.min.z().min(box1.min.z())
            ),
            Point3::from_values(
                box0.max.x().max(box1.max.x()),
                box0.max.y().max(box1.max.y()),
                box0.max.z().max(box1.max.z())
            )
        )
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let inv_dir = 1.0 / ray.direction()[axis];
            let t0 = (self.min[axis] - ray.origin()[axis]) * inv_dir;
            let t1 = (self.max[axis] - ray.origin()[axis]) * inv_dir;

            let (t0, t1) = if inv_dir < 0.0 { (t1, t0) } else { (t0, t1) };

            ray_t.min = ray_t.min.max(t0);
            ray_t.max = ray_t.max.min(t1);

            if ray_t.max < ray_t.min {
                return false;
            }
        }
        true
    }
}

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>) -> Self {
        let axis = ((random_float() * 3.0) as usize).min(2);
        
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        match objects.len() {
            1 => {
                let bbox = objects[0].bounding_box();
                BVHNode {
                    left: objects[0].clone(),
                    right: objects[0].clone(),
                    bbox,
                }
            }
            2 => {
                objects.sort_by(comparator);
                let bbox = AABB::from_boxes(&objects[0].bounding_box(), &objects[1].bounding_box());
                BVHNode {
                    left: objects[0].clone(),
                    right: objects[1].clone(),
                    bbox,
                }
            }
            _ => {
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                let left_objects = objects[..mid].to_vec();
                let right_objects = objects[mid..].to_vec();
                
                let left = Arc::new(BVHNode::new(left_objects));
                let right = Arc::new(BVHNode::new(right_objects));
                let bbox = AABB::from_boxes(&left.bounding_box(), &right.bounding_box());
                
                BVHNode { left, right, bbox }
            }
        }
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        a.bounding_box().min.x().partial_cmp(&b.bounding_box().min.x()).unwrap_or(Ordering::Equal)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        a.bounding_box().min.y().partial_cmp(&b.bounding_box().min.y()).unwrap_or(Ordering::Equal)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        a.bounding_box().min.z().partial_cmp(&b.bounding_box().min.z()).unwrap_or(Ordering::Equal)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(ray, ray_t, rec);
        let right_t_max = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self.right.hit(ray, Interval::from_range(ray_t.min, right_t_max), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}