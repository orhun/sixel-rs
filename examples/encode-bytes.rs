use image::GenericImageView;
use sixel_rs::{
    optflags::DiffusionMethod,
    pixelformat::PixelFormat,
    sixel_string,
    status::{Error, Status},
};
use std::path::Path;

fn main() -> Status<()> {
    // Load the image from a file using the `image` crate
    let img_path = Path::new("examples/test.png");
    let img = image::open(img_path).map_err(|_| Error::Other)?;

    // Convert the image to RGB8 format (8 bits per channel)
    let img_rgb = img.to_rgb8();

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Convert the image to raw byte data
    let bytes: Vec<u8> = img_rgb
        .pixels()
        .flat_map(|pixel| pixel.0.to_vec())
        .collect();

    // Call the sixel_string function
    let sixel_output = sixel_string(
        &bytes,
        width as i32,
        height as i32,
        PixelFormat::RGB888,
        DiffusionMethod::Atkinson,
    )?;

    // Print the generated SIXEL string
    println!("{sixel_output}");

    Ok(())
}
