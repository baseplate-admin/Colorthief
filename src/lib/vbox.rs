use std::collections::HashMap;
struct VBox;

impl VBox {
    pub fn new(
        &self,
        r1: i64,
        r2: i64,
        g1: i64,
        g2: i64,
        b1: i64,
        b2: i64,
        histro: HashMap<i64, i64>,
    ) {
        self.r1 = r1;
        self.r2 = r2;
        self.g1 = g1;
        self.g2 = g2;
        self.b1 = b1;
        self.b2 = b2;
        self.histo = histo;
    }

    pub fn volume(&self) {
        sub_r = self.r2 - self.r1;
        sub_g = self.g2 - self.g1;
        sub_b = self.b2 - self.b1;
        return (sub_r + 1) * (sub_g + 1) * (sub_b + 1);
    }
}
