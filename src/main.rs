use lib::mccq;
fn main() {
    println!("Hello, world!");
    mccq::MMCQ::get_histo([(123, 123, 4)])
}
