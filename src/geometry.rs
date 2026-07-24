pub fn abs_difference(x: i32, y: i32) -> i32 {
    if x >= y { x - y } else { y - x }
}

pub fn distance_squared(x_a: i32, y_a: i32, x_b: i32, y_b: i32) -> i32 {
    let dx: i32 = abs_difference(x_a, x_b);
    let dy: i32 = abs_difference(y_a, y_b);
    (dx * dx) + (dy * dy)
}

pub fn distance_from_origin_squared(x: i32, y: i32) -> i32 {
    distance_squared(x, y, 0, 0)
}

pub fn closest_point(ax: i32, ay: i32, bx: i32, by: i32) -> &'static str {
    let point_a: i32 = distance_from_origin_squared(ax, ay);
    let point_b: i32 = distance_from_origin_squared(bx, by);

    if point_a > point_b {
        "B"
    } else if point_b > point_a {
        "A"
    } else {
        "TIE"
    }
}