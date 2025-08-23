use image::{ImageReader, ImageResult, RgbImage};

use crate::utility::{
    color::Color,
    interval::Interval,
    vec3::{Point3, Precision},
};

use super::texture::Texture;

#[derive(Clone)]
pub struct ImageTexture(image::RgbImage);

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image = ImageTexture::open_image(filename).unwrap_or_else(|_| {
            eprintln!("Image not found :C");
            RgbImage::new(0, 0)
        });

        Self(image)
    }

    fn open_image(filename: &str) -> ImageResult<RgbImage> {
        Ok(ImageReader::open(filename)?
            .with_guessed_format()?
            .decode()?
            .to_rgb8())
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: Precision, v: Precision, _p: Point3) -> Color {
        // If there's no image, return solid cyan for debugging
        if self.0.height() == 0 {
            return Color::new(0., 1., 1.);
        }

        // clamp coordinates to [0,1] x [1,0]
        let u = Interval::new(0., 1.).clamp(u);
        let v = 1. - Interval::new(0., 1.).clamp(v);

        let i = u * (self.0.width()-1) as Precision;
        let j = v * (self.0.height()-1) as Precision;

        let pixel = self.0.get_pixel(i as u32, j as u32).0;
        let mut p = [0.; 3];
        for (i, val) in pixel.iter().enumerate() {
            let v = *val as f32 / 255.;
            // Convert from sRGB to linear
            p[i] = v.powf(2.2);
        }

        Color::new(p[0], p[1], p[2])
    }
}
