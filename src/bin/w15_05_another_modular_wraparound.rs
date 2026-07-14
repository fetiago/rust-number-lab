fn main() {
    let current: u16 = 62137;
    let step: u16 = 7224;

    let remaining_until_max: u16 = 65535 - current; // bc our modulus is 65536
    let result = if step > remaining_until_max {
        step - remaining_until_max - 1
    } else {
        current + step
    };

    println!("{result}");
}