// what is your height in meters ?

fn main() {
    let cm_height: u8 = 108;

    println!(
        "your height is {}.{:02}m.",
        cm_height / 100,
        cm_height % 100
    );
}
