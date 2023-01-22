use crate::primitives::ray::Ray;

use super::{Hittable, HitRecord};


pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            list: Vec::new(),
        }
    } 

    pub fn push(&mut self, obj: Box<dyn Hittable>) {
        self.list.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = std::f64::MAX;
        let mut hit_result = None;

        for obj in &self.list {
            let record = obj.hit(ray, t_min, t_max);
            match record {
                Some(x) => {
                    if x.t < closest {
                        closest = x.t;
                        hit_result = Some(x);
                    }
                },
                None => (),
            }
        }

        hit_result
    }
}