#[path = "w2_01_abs_difference_fn.rs"]
mod abs;

pub fn distance_squared(x_a: i32, y_a: i32, x_b: i32, y_b: i32) -> i32 {
    let dx: i32 = abs::abs_difference(x_a, x_b);
    let dy: i32 = abs::abs_difference(y_a, y_b);
    (dx * dx) + (dy * dy)
}

fn main() {
    let x1: i32 = 4;
    let y1: i32 = 5;
    let x2: i32 = 1;
    let y2: i32 = 1;

    println!("{}", distance_squared(x1, y1, x2, y2));
}
