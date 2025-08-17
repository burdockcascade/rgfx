use crate::graphics::color::Color;
use image::{ImageReader, RgbaImage};

#[derive(Clone, Debug)]
pub struct Image {
    pub path: String,
    pub image: image::DynamicImage
}

impl Image {

    pub fn from_file(path: &str) -> Self {

        let dynamic_image = ImageReader::open(path)
            .expect("Failed to open image file")
            .decode()
            .expect("Failed to decode image file");

        Self {
            path: path.to_string(),
            image: dynamic_image
        }
    }

    pub fn single_pixel(color: Color) -> Self {
        let mut img = RgbaImage::new(1, 1);
        img.put_pixel(0, 0, image::Rgba(color.into()));
        Self {
            path: format!("single_pixel_{:?}_{:?}_{:?}_{:?}", color.r, color.g, color.b, color.a),
            image: image::DynamicImage::ImageRgba8(img)
        }
    }

    pub fn write_to_file(&self, path: &str) {
        self.image.save(path).unwrap();
    }

}