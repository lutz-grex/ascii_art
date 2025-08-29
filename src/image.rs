use anyhow::anyhow;
use image::imageops::{resize, FilterType};
use image::{GrayImage, ImageReader};

pub fn get_gray_image(input_path: &str) -> anyhow::Result<GrayImage> {
    let img = ImageReader::open(input_path).map_err(|e| anyhow!("Can't read image: {e}"))?;
    let decode_img = img
        .decode()
        .map_err(|e| anyhow!("Can't decode image {e}"))?;
    Ok(decode_img.to_luma8())
}

pub fn resize_gray_image(img: &GrayImage, target_width: f32, target_height: f32) -> GrayImage {
    let t = target_height.round() as u32;
    let w = target_width.round() as u32;

    resize(img, w, t, FilterType::Nearest)
}
