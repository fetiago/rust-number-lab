#[path = "w2_02_grade_classifier_fn.rs"]
mod grade_classifier;

fn is_valid_grade(n: u8) -> bool { n <= 100 }

fn main() {
    let arr: [u8; 11] = [59, 60, 69, 70, 79, 80, 89, 90, 100, 101, 255];
    let mut i: usize = 0;
    while i < arr.len() {
        if is_valid_grade(arr[i]) {
            println!("{}", grade_classifier::classify_grade(arr[i]));
        }
        i += 1;
    }
}