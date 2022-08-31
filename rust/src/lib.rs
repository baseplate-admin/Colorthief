use image;
use palette_extract::get_palette_rgb;
pub use palette_extract::Color;

pub fn get_pallete(src: String) -> Vec<Color> {
    let image_path = src;

    // open and decode the image using the `image` crate
    let img = image::open(image_path).unwrap();

    // grab a reference to the underlying pixels/RGB buffer
    let pixels = img.as_bytes();

    // extract the color palette
    let palette = get_palette_rgb(&pixels);

    // output the extracted color palette
    return palette;
}
