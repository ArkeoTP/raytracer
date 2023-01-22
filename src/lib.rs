use std::io::{self, Write};

use objs::{hittablelist::HittableList};
use primitives::{ray::{Ray}, vector::*};
use crate::{image::{Image}, objs::Hittable};
use crate::camera::Camera;

pub mod vector_generic;
pub mod image;
pub mod camera;
pub mod primitives;
pub mod objs;
pub mod math;

fn ray_color(ray: &Ray, objects: &HittableList) -> Color {
    let record = objects.hit(ray, 0.0, f64::MAX);
    match record {
        Some(r) => 
        0.5 * Color::new(r.normal.0 + 1.0, r.normal.1 + 1.0, r.normal.2 + 1.0),
        None => {
        let unit_direction = ray.dir.unit_vector();
        let t = 0.5*(unit_direction.1 + 1.0);

        (1.0 - t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
        }
    } 
}


pub fn render(image: &mut Image, camera: &Camera, objects: &HittableList) {

    let (image_width, image_height) = (image.width, image.height);

    for y in 0..image_height {
        print!("{esc}c", esc = 27 as char);
        io::stdout().flush().unwrap();
        eprint!("Writing line {y} out of {image_height}\n");
        io::stderr().flush().unwrap();
        for x in 0..image_width {
            let u = f64::from(x) / f64::from(image_width - 1);
            let v = f64::from(y) / f64::from(image_height - 1);
            
            let ray = Ray {
                orig: camera.origin,
                dir: camera.lower_left + u * camera.horizontal + v * camera.vertical - camera.origin
            };
            
            let pixel = ray_color(&ray, &objects);

            image.put_pixel(x, image_height - y - 1, pixel);
        }
    }
    eprint!("Done!");

}