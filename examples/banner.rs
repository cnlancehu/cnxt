use anyhow::Result;
use cnxt::{ColoredString, Colorize as _};
use crossterm::terminal::size;
use image::{
    GenericImageView as _, Rgba, imageops::FilterType, load_from_memory,
};

fn main() {
    #[cfg(windows)]
    cnxt::control::set_virtual_terminal(true);

    let buffer = include_bytes!("../assets/banner.png");
    let lines = img2lines(buffer).unwrap();
    for line in lines {
        println!("{}", line);
    }
}

fn img2lines(buffer: &[u8]) -> Result<Vec<String>> {
    let mut image = load_from_memory(buffer)?;
    let (width, height) = image.dimensions();
    let termsize = size().unwrap();

    // Calculate zoom factor based on terminal size
    // Each character takes 2 rows of pixels due to the half-block approach
    let term_width = termsize.0 as f64;
    let term_height = (termsize.1 * 2) as f64; // Each terminal row can display 2 pixels vertically using half blocks
    let width_ratio = term_width / width as f64;
    let height_ratio = term_height / height as f64;
    let zoom = width_ratio.min(height_ratio); // Use the smaller ratio to ensure it fits in both dimensions

    image = image.resize(
        (width as f64 * zoom) as u32,
        (height as f64 * zoom) as u32,
        FilterType::CatmullRom,
    );
    let pixels = image.pixels().map(|p| p).collect::<Vec<_>>();
    let mut pixels_2d: Vec<Vec<Rgba<u8>>> = Vec::new();
    for pixel in pixels {
        let (x, y) = (pixel.0, pixel.1);
        if x == 0 {
            pixels_2d.push(Vec::new());
        };
        pixels_2d.last_mut().unwrap().push(image.get_pixel(x, y));
    }
    let pixel_2d_pairs: Vec<(Vec<Rgba<u8>>, Option<Vec<Rgba<u8>>>)> = pixels_2d
        .chunks(2)
        .map(|chunk| {
            let row1 = chunk[0].clone();
            let row2 = if chunk.len() > 1 {
                Some(chunk[1].clone())
            } else {
                None
            };
            (row1, row2)
        })
        .collect();

    let mut lines: Vec<String> = Vec::new();
    for (row1, row2) in pixel_2d_pairs {
        let mut line = String::new();
        if let Some(row2) = row2 {
            for i in 0..row1.len() - 1 {
                let block: ColoredString = "▀"
                    .truecolor(row1[i][0], row1[i][1], row1[i][2])
                    .on_truecolor(row2[i][0], row2[i][1], row2[i][2]);
                line = format!("{}{}", line, block);
            }
        }
        lines.push(line);
    }
    Ok(lines)
}
