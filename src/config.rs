use crate::cli::Cli;

pub struct Config<'a> {
    input_path: &'a str,
    output_path: &'a str,
    preview: bool,
    width: f32,
}

impl<'a> Config<'a> {
    pub fn new(input_path: &'a str, output_path: &'a str, preview: bool, width: f32) -> Self {
        Self {
            input_path,
            output_path,
            preview,
            width,
        }
    }

    pub fn from_cli(cli: &'a Cli) -> Self {
        let target_width = cli.target_width.unwrap_or(100.0);
        Self {
            input_path: cli.image.as_str(),
            output_path: cli.output.as_str(),
            preview: cli.preview,
            width: target_width,
        }
    }

    pub fn input_path(&self) -> &'a str {
        self.input_path
    }

    pub fn output_path(&self) -> &'a str {
        self.output_path
    }

    pub fn preview(&self) -> bool {
        self.preview
    }

    pub fn width(&self) -> f32 {
        self.width
    }
}
