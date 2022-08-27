use std::collections::HashMap;

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
    let hashmap = HashMap::from([
        (String::from("Color"), cmap::Value::Color(12, 312, 321)),
        (String::from("Color"), cmap::Value::Color(123, 321, 312)),
    ]);
    let x = cmap::CMap::new(vec![hashmap]);
    println!("Found {:?}", x.palette())
}
