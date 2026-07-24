pub fn is_valid_grade(n: u8) -> bool {
    n <= 100
}

pub fn classify_grade(n: u8) -> &'static str {
    if n < 60 {
        "F"
    } else if n < 70 {
        "D"
    } else if n < 80 {
        "C"
    } else if n < 90 {
        "B"
    } else if n <= 100 {
        "A"
    } else {
        "N/A"
    }
}