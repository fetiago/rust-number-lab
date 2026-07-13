// distance between 2 points in 2D space

fn main() {
    let x1: i32 = 2;
    let y1: i32 = 3;

    let x2: i32 = 7;
    let y2: i32 = 11;

    let dx: i32 = x2 - x1;
    let dy: i32 = y2 - y1;
    let square_distance: i32 = (dx * dx) + (dy * dy);
    let sq_distance: f32 = square_distance as f32;

    println!("the distance is {:.2}", sq_distance.sqrt());
}
