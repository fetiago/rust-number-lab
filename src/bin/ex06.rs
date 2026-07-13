// distance in 1D

fn main() {
    let a: i32 = 3;
    let b: i32 = -10;

    let distance = if b - a < 0 { -(b - a) } else { b - a };
    println!("{distance}");
}
