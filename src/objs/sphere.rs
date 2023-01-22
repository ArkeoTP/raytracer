use crate::primitives::vector::Point3;

use super::hittable::*;
use crate::primitives::vector::*;



pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::primitives::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = Vec3::dot(ray.dir, ray.dir);
        let b_over_2 = Vec3::dot(oc, ray.dir);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b_over_2 * b_over_2 - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (- b_over_2 - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (- b_over_2 + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center).unit_vector();
        Some(
            HitRecord {
                t: root,
                p: ray.at(root),
                normal: normal,
            }
        )
        
    }
}