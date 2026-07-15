fn main() {
    // 0 <= a, b <= 255
    let a: u8 = 5;
    let b: u8 = 13;

    println!("given:\n  a = {}\n  b = {}\nthen:\n", a, b);
    // safe_subtraction
    let diff: u8 = {
        if a >= b {
            a - b
        } else {
            b - a
        }
    };
    println!("the difference between a and b is {}", diff);
}