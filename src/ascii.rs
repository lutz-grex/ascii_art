use crate::AsciiImage;
use std::fs::OpenOptions;
use std::io::Write;

pub fn save_ascii(ascii_img: &AsciiImage, output_path: &str) -> anyhow::Result<()> {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .truncate(true)
        .open(output_path)
    {
        for row in ascii_img {
            let line: String = row.into_iter().collect();
            let _ = writeln!(file, "{}", line);
        }
    }

    Ok(())
}

pub fn print_ascii(ascii_img: &AsciiImage) {
    for row in ascii_img {
        let line: String = row.into_iter().collect();
        println!("{}", line);
    }
}
