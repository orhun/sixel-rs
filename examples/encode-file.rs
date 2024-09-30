use sixel_rs::{
    encoder::Encoder,
    optflags::{DiffusionMethod, Quality},
    status::{Error, Status},
};
use std::path::Path;

fn main() -> Status<()> {
    // Create a new Encoder instance
    let encoder = Encoder::new()?;

    // Set some encoding options
    encoder.set_num_colors(255)?;
    encoder.set_diffusion(DiffusionMethod::Atkinson)?;
    encoder.set_quality(Quality::High)?;

    // Specify the output file
    let output_path = Path::new("output.sixel");
    encoder.set_output(output_path)?;

    // Encode an image file
    let source_image = Path::new("examples/test.png");
    if !source_image.exists() {
        return Err(Error::BadArgument);
    }
    encoder.encode_file(source_image)?;

    println!("Encoding completed successfully!");

    Ok(())
}
