use crate::vbox;

use std::{
    cmp::{max, min},
    collections::HashMap,
};
pub struct MMCQ;

pub static SIGBITS: i64 = 5;
pub static RSHIFT: i64 = 8 - SIGBITS;
pub static MAX_ITERATION: i64 = 1000;
pub static FRACT_BY_POPULATIONS: f64 = 0.75;

impl MMCQ {
    pub fn get_color_index(r: i64, g: i64, b: i64) -> i64 {
        return (r << (2 * SIGBITS)) + (g << SIGBITS) + b;
    }

    pub fn get_histo(pixels: Vec<(i64, i64, i64)>) -> HashMap<i64, i64> {
        /*
            histo (1-d array, giving the number of pixels in each quantized
            region of color space)
        */
        let mut histo: HashMap<i64, i64> = HashMap::new();

        for pixel in pixels {
            let rval = pixel.0 >> RSHIFT;
            let gval = pixel.1 >> RSHIFT;
            let bval = pixel.2 >> RSHIFT;
            let index = MMCQ::get_color_index(rval, gval, bval);
            // Optimize this
            // Update  dict at index key with 0 and add 1

            histo.insert(index, *histo.entry(index).or_insert(0) + 1);
        }
        return histo;
    }

    pub fn vbox_from_pixels(pixels: Vec<(i64, i64, i64)>, histo: HashMap<i64, i64>) -> vbox::VBox {
        let mut rmin = 1000000;
        let mut rmax = 0;
        let mut gmin = 1000000;
        let mut gmax = 0;
        let mut bmin = 1000000;
        let mut bmax = 0;

        for pixel in pixels {
            let rval = pixel.0 >> RSHIFT;
            let gval = pixel.1 >> RSHIFT;
            let bval = pixel.2 >> RSHIFT;
            rmin = min(rval, rmin);
            rmax = max(rval, rmax);
            gmin = min(gval, gmin);
            gmax = max(gval, gmax);
            bmin = min(bval, bmin);
            bmax = max(bval, bmax);
        }
        //  VBox(rmin, rmax, gmin, gmax, bmin, bmax, histo)
        return vbox::VBox::new(rmin, rmax, gmin, gmax, bmin, bmax, histo);
    }

    pub fn median_cut_apply(
        histo: HashMap<i64, i64>,
        vbox: (i64, i64, i64, i64, i64, i64, HashMap<i64, i64>),
    ) -> bool {
        return true;
    }
}
