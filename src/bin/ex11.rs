// secure mutability

fn main() {
    let mut _total = 0;

    _total += 10;
    _total += 15;
    _total -= 6;

    println!("{_total}");

    let n = 5;
    let n = n + 1;
    let n = n * 2;
    let n = n % 5;
    // let n = "stone";

    println!("{n}");
}
