fn main() {
    // point A
    let x1: i32 = 10;
    let y1: i32 = 1;

    // point B
    let x2: i32 = 3;
    let y2: i32 = 4;

    let point_a: i32 = x1 * x1 + y1 * y1;
    let point_b: i32 = x2 * x2 + y2 * y2;

    if point_a == point_b {
        println!("they have the same distance to origin");
    } else {
        if point_a > point_b {
            println!("B is closer to origin");
        } else {
            println!("A is closer to origin");
        }
    }
}
