// y - yo / x - xo

fn main() {
    let x1: i32 = 3;
    let y1: i32 = 12;

    let x2: i32 = 1;
    let y2: i32 = -1;

    let dx = x2 - x1;
    let dy = y2 - y1;

    if dx == 0 {
        println!("slope is undefined");
    } else {
        let m: f32 = dy as f32 / dx as f32;
        println!("m = {m}");
    }
}
