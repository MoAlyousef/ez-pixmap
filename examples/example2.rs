extern crate ez_pixmap;
extern crate image;

#[path = "rustlogo.rs"]
mod rustlogo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_image = ez_pixmap::RgbaImage::new(rustlogo::RUSTLOGO)?;
    image::save_buffer(
        "examples/image2.png",
        my_image.data(),
        my_image.width(),
        my_image.height(),
        image::ColorType::Rgba8,
    )?;
    Ok(())
}
