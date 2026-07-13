// a point in 2D

fn main() {
    let x: i32 = -4;
    let y: i32 = 7;

    println!("point: ({},{})", x, y);

    if x == 0 && y == 0 {
        println!("point on origin");
    } else if x == 0 {
        println!("point on y");
    } else if y == 0 {
        println!("point on x");
    } else if x > 0 && y > 0 {
        println!("point on 1st quadrant");
    } else if x < 0 && y > 0 {
        println!("point on 2nd quadrant");
    } else if x < 0 && y < 0 {
        println!("point on 3rd quadrant");
    } else {
        println!("point on 4th quadrant");
    }
}
