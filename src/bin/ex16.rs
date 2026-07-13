// compact height formatter

fn main() {
    let cm_height: u8 = 178;
    println!("{}.{:02}m", cm_height / 100, cm_height % 100);
    // println!("{},{:02}m", cm_height / 100, cm_height % 100);
}
