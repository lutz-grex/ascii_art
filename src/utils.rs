use crate::{AsciiImage, ASCII_RANGE};
use image::GrayImage;

pub mod calculate {
    use super::*;

    pub fn calculate_target_height(
        img: &GrayImage,
        target_width: f32,
        aspect_correction: f32,
    ) -> f32 {
        let original_height = img.height() as f32;
        let original_width = img.width() as f32;
        original_height * target_width / original_width * aspect_correction
    }
}

pub mod convert {
    use super::*;
    pub fn convert_img_to_ascii(img: &GrayImage) -> AsciiImage {
        let mut ascii_img: AsciiImage =
            vec![vec![' '; img.width() as usize]; img.height() as usize];

        for (x, y, pixel) in img.enumerate_pixels() {
            let brightness = pixel.0[0];
            let index = (brightness as usize * ASCII_RANGE.len()) / 256;
            ascii_img[y as usize][x as usize] = ASCII_RANGE[index];
        }

        ascii_img
    }
}
