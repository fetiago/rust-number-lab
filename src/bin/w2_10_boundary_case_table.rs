#[path = "w2_02_grade_classifier_fn.rs"]
mod grade;
#[path = "w2_03_valid_grade_domain.rs"]
mod valid;

fn grader(n: u8) {
    if valid::is_valid_grade(n) {
        println!(
            "Grade {} -> valid: {}, classification: {}",
            n,
            valid::is_valid_grade(n),
            grade::classify_grade(n)
        );
    } else {
        println!("Grade {} -> valid: {}", n, valid::is_valid_grade(n));
    }
}
fn main() {
    grader(59);
    grader(60);
    grader(69);
    grader(70);
    grader(79);
    grader(80);
    grader(89);
    grader(90);
    grader(100);
    grader(101);
    grader(255);
}
