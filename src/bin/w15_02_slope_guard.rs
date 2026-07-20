fn main() {
    let x1: i32 = 12;
    let y1: i32 = 24;

    let x2: i32 = 0;
    let y2: i32 = -6;

    let dx: i32 = x2 - x1;
    let dy: i32 = y2 - y1;

    if dx == 0 {
        println!("slope is undefined");
    } else {
        let m: f32 = dy as f32 / dx as f32;
        println!("m = {m}");
    }
}
