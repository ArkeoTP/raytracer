use image::{ImageBuffer, Rgb, RgbImage};

pub struct Image {
    pub width: u32,
    pub height: u32,    
    image: ImageBuffer<Rgb<u8>, Vec<u8>>, 
}

impl Image {

pub fn new(image_width: u32, image_height: u32) -> Image {
    let (image_width, image_height) = (image_width, image_height);
    
    Image {
        width: image_width,
        height: image_height,
        image: RgbImage::new(image_width, image_height),
    }
}

pub fn from_width(image_width: u32, aspect_ratio: f64) -> Image {
    let image_height = image_width as f64 / aspect_ratio;
    Self::new(image_width, image_height as u32)
}



/// Puts pixel at location `(x, y)`
/// 
/// # Panics
/// 
/// if `(x, y)` is out of bounds for `(height, width)`
///
pub fn put_pixel(&mut self, x: u32, y: u32, pixel: crate::primitives::vector::Vec3) {
    self.image.put_pixel(x, y, Rgb::from(pixel));
}

pub fn save(&self, path: &str) {
    self.image.save(path).unwrap();
}

}