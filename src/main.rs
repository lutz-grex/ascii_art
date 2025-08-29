mod ascii;
mod cli;
mod config;
mod image;
mod utils;

use crate::ascii::{print_ascii, save_ascii};
use crate::cli::Cli;
use crate::config::Config;
use crate::image::{get_gray_image, resize_gray_image};
use crate::utils::calculate::calculate_target_height;
use crate::utils::convert::convert_img_to_ascii;
use anyhow::Result;
use clap::Parser;

type AsciiImage = Vec<Vec<char>>;

const ASCII_RANGE: [char; 10] = ['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from_cli(&cli);
    generate_ascii_art(&config)?;

    Ok(())
}

fn generate_ascii_art(config: &Config) -> Result<()> {
    let gray_img = get_gray_image(config.input_path())?;
    let target_height = calculate_target_height(&gray_img, config.width(), 0.5);
    let resized_gray_img = resize_gray_image(&gray_img, config.width(), target_height);
    let ascii_img = convert_img_to_ascii(&resized_gray_img);

    if config.preview() {
        print_ascii(&ascii_img);
    }

    save_ascii(&ascii_img, config.output_path())?;

    Ok(())
}
