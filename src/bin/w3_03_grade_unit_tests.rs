use rust_number_lab::grade::is_valid_grade;
use rust_number_lab::grade::classify_grade;

const VALID: bool = true;
const INVALID: bool = false;

#[test]
fn validation_works() {
    assert_eq!(is_valid_grade(59), VALID);
    assert_eq!(is_valid_grade(60), VALID);
    assert_eq!(is_valid_grade(69), VALID);
    assert_eq!(is_valid_grade(70), VALID);
    assert_eq!(is_valid_grade(79), VALID);
    assert_eq!(is_valid_grade(80), VALID);
    assert_eq!(is_valid_grade(89), VALID);
    assert_eq!(is_valid_grade(90), VALID);
    assert_eq!(is_valid_grade(100), VALID);
    assert_eq!(is_valid_grade(101), INVALID);
    assert_eq!(is_valid_grade(255), INVALID);
}

#[test]
fn classifing_works() {
    assert_eq!(classify_grade(59), "F");
    assert_eq!(classify_grade(60), "D");
    assert_eq!(classify_grade(69), "D");
    assert_eq!(classify_grade(70), "C");
    assert_eq!(classify_grade(79), "C");
    assert_eq!(classify_grade(80), "B");
    assert_eq!(classify_grade(89), "B");
    assert_eq!(classify_grade(90), "A");
    assert_eq!(classify_grade(100), "A");
    assert_eq!(classify_grade(101), "N/A");
    assert_eq!(classify_grade(255), "N/A");
}