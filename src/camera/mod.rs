use crate::primitives::vector::{Vec3, Point3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left:  Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn from_zero_origin(viewport_width: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let origin = Vec3::zeros();
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length); 
        
        Camera {
            origin: origin,
            lower_left: lower_left,
            horizontal: horizontal,
            vertical: vertical,            
        }
    }
}