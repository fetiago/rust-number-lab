fn main() {
    let grade: u8 = 78;

    if grade < 60 {
        println!("F");
    } else if grade >= 60 && grade < 70 {
        println!("D");
    } else if grade >= 70 && grade < 80 {
        println!("C");
    } else if grade >= 80 && grade < 90 {
        println!("B");
    } else {
        println!("A");
    }
}
