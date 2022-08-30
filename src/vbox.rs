use crate::mccq;

use std::collections::HashMap;
pub struct VBox {
    r1: i64,
    r2: i64,
    g1: i64,
    g2: i64,
    b1: i64,
    b2: i64,
    histo: HashMap<i64, i64>,
}

impl VBox {
    pub fn new(
        r1: i64,
        r2: i64,
        g1: i64,
        g2: i64,
        b1: i64,
        b2: i64,
        histo: HashMap<i64, i64>,
    ) -> Self {
        return Self {
            r1,
            r2,
            g1,
            g2,
            b1,
            b2,
            histo,
        };
    }

    pub fn volume(&self) -> i64 {
        let sub_r = self.r2 - self.r1;
        let sub_g = self.g2 - self.g1;
        let sub_b = self.b2 - self.b1;
        return (sub_r + 1) * (sub_g + 1) * (sub_b + 1);
    }

    pub fn copy(mut self) -> Self {
        return Self {
            r1: self.r1,
            r2: self.r2,
            g1: self.g1,
            g2: self.g2,
            b1: self.b1,
            b2: self.b2,
            histo: self.histo,
        };
    }
    pub fn avg(mut self) -> (f64, f64, f64) {
        let mut ntot = 0.00;
        let mut mult = (1 << (8 - mccq::SIGBITS)) as f64;
        let mut r_sum = 0.00;
        let mut g_sum = 0.00;
        let mut b_sum = 0.00;
        for i in self.r1..self.r2 + 1 {
            for j in self.g1..self.g2 + 1 {
                for k in self.b1..self.b2 + 1 {
                    let histoindex = mccq::MMCQ::get_color_index(i, j, k);
                    let hval = *self.histo.entry(histoindex).or_insert(0) as f64;
                    ntot += hval;
                    r_sum += hval * (i as f64 + 0.5) * mult;
                    g_sum += hval * (j as f64 + 0.5) * mult;
                    b_sum += hval * (k as f64 + 0.5) * mult;
                }
            }
        }

        let mut r_avg;
        let mut g_avg;
        let mut b_avg;

        if ntot != 0.00 {
            r_avg = r_sum / ntot;
            g_avg = g_sum / ntot;
            b_avg = b_sum / ntot;
        } else {
            r_avg = mult * ((self.r1 + self.r2 + 1) / 2) as f64;
            g_avg = mult * ((self.g1 + self.g2 + 1) / 2) as f64;
            b_avg = mult * ((self.b1 + self.b2 + 1) / 2) as f64;
        }
        return (r_avg, g_avg, b_avg);
    }

    pub fn contains(&self, pixel: (i64, i64, i64)) -> bool {
        let rval = pixel.0 >> mccq::RSHIFT;
        let gval = pixel.1 >> mccq::RSHIFT;
        let bval = pixel.2 >> mccq::RSHIFT;

        // https://stackoverflow.com/questions/53968385/how-can-i-define-a-function-that-returns-true-if-every-value-in-an-iterator-is-t
        return [
            rval >= self.r1,
            rval <= self.r2,
            gval >= self.g1,
            gval <= self.g2,
            bval >= self.b1,
            bval <= self.b2,
        ]
        .into_iter()
        .all(|x| !!x);
    }

    pub fn count(mut self) -> i64 {
        let mut npix = 0;
        for i in self.r1..self.r2 + 1 {
            for j in self.g1..self.g2 + 1 {
                for k in self.b1..self.b2 + 1 {
                    let index = mccq::MMCQ::get_color_index(i, j, k);
                    npix += *self.histo.entry(index).or_insert(0)
                }
            }
        }
        return npix;
    }
}
