#[path = "w2_06_distance_squared_fn.rs"]
mod ds;

fn distance_from_origin_squared(x: i32, y: i32) -> i32 {
    ds::distance_squared(x, y, 0, 0)
}

fn closest_point(ax: i32, ay: i32, bx: i32, by: i32) -> &'static str {
    let point_a: i32 = distance_from_origin_squared(ax, ay);
    let point_b: i32 = distance_from_origin_squared(bx, by);

    if point_a > point_b {
        "A"
    } else if point_b > point_a {
        "B"
    } else {
        "TIE"
    }
}

fn main() {
    let point_a_x: i32 = -1;
    let point_a_y: i32 = 1;
    let point_b_x: i32 = -1;
    let point_b_y: i32 = -1;

    println!(
        "{}",
        closest_point(point_a_x, point_a_y, point_b_x, point_b_y)
    );
}
