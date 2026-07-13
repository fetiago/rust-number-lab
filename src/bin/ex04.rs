// is it positive ?

fn main() {
    let score: i8 = -37;

    if score == 0 {
        println!("the score is neither positive or negative");
    } else {
        if score > 0 {
            println!("the score is positive");
        } else {
            println!("the score is negative");
        }
    }
}
