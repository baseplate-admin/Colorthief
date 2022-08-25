mod cmap;
mod mccq;
mod vbox;
/**
    Source to source translation from Python implementation ( https://github.com/fengsp/color-thief-py/blob/master/colorthief.py )
    Basic Rust port of the MMCQ (modified median cut quantization)
    algorithm from the Leptonica library (http://www.leptonica.com/).
*/
fn main() {
    println!("Hello, world!");
    mccq::MMCQ::get_histo(vec![(123, 123, 4), (123, 123, 4)]);
}
