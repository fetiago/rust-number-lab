// is it divisible by 3, 5 and 10 ?

fn main() {
    let n: i32 = 45;

    println!("Is it divisible by 3? {}", n % 3 == 0);
    println!("Is it divisible by 5? {}", n % 5 == 0);
    println!("Is it divisible by 10? {}", n % 10 == 0);
}
