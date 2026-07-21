pub fn abs_difference(x: i32, y: i32) -> i32 {
    if x >= y { x - y } else { y - x }
}

fn main() {
    let a: i32 = -3;
    let b: i32 = 10;

    let difference: i32 = abs_difference(a, b);
    println!("{}", difference);
}
