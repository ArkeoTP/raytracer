use raytracer::objs::hittablelist::HittableList;
use raytracer::objs::sphere::Sphere;
use raytracer::render;
use raytracer::image::Image;
use raytracer::camera::Camera;
use raytracer::primitives::vector::Point3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let path = "image.png";
    let mut img = Image::from_width(image_width, aspect_ratio);


    // World
    let mut world = HittableList::new();
    world.push(Box::new(
        Sphere {center: Point3::new(0.0, 0.0, -1.0), radius: 0.5, }));
    world.push(Box::new(
        Sphere {center: Point3::new(0.0, -100.5, -1.0), radius: 100.0, }));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let camera = Camera::from_zero_origin(viewport_width, viewport_height, focal_length);

    // Render
    render(&mut img, &camera, &world);

    img.save(path);


}
