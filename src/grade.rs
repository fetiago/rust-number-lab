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

pub fn grader(n: u8) {
    if is_valid_grade(n) {
        println!(
            "Grade {} -> valid: {}, classification: {}",
            n,
            is_valid_grade(n),
            classify_grade(n)
        );
    } else {
        println!("Grade {} -> valid: {}", n, is_valid_grade(n));
    }
}