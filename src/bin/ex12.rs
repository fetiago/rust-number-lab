// inside blocks

fn main() {
    let k = 2;

    let x = {
        let a = 5;
        let b = 10;

        a * b * k
    };

    let y = {
        let a = 6;
        let b = 12;

        a * b * k
    };

    println!("{} and {}", x, y);
}
