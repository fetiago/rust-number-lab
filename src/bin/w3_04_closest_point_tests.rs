use rust_number_lab::geometry::closest_point;


#[test]
fn closest_point_works() {
    assert_eq!(closest_point(1, 1, 5, 5), "A");
    assert_eq!(closest_point(5, 5, 1, 1), "B");

        // Equal distances
    assert_eq!(closest_point(1, 1, -1, -1), "TIE");
    assert_eq!(closest_point(3, 4, 0, 5), "TIE");
    assert_eq!(closest_point(0, 0, 0, 0), "TIE");

        // One point is the origin
    assert_eq!(closest_point(0, 0, 1, 1), "A");
    assert_eq!(closest_point(10, 10, 0, 0), "B");

        // Points on axes
    assert_eq!(closest_point(2, 0, 3, 0), "A");
    assert_eq!(closest_point(0, 7, 0, 4), "B");
    assert_eq!(closest_point(-2, 0, 0, 2), "TIE");

        // Negative coordinates
    assert_eq!(closest_point(-1, -1, -5, -5), "A");
    assert_eq!(closest_point(-10, -10, -2, -2), "B");
    assert_eq!(closest_point(-3, -4, 6, 8), "A");

        // Mixed coordinates
    assert_eq!(closest_point(1, -2, -4, 5), "A");
    assert_eq!(closest_point(-6, 8, 3, -4), "B");
    assert_eq!(closest_point(4, -3, -4, 3), "TIE");

        // Same point
    assert_eq!(closest_point(2, 3, 2, 3), "TIE");
}