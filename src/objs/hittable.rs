use crate::primitives::{vector::{Point3, Vec3}, ray::Ray};

///
/// Struct containing Hit information
/// 
/// `p`: 3D coordinate of the hit
/// 
/// `normal`: normal vector
/// 
/// `t`: distance from the source
///  
#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; 
}
