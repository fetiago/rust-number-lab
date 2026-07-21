#[path = "w2_04_caesar_encode_fn.rs"]
mod encode;

fn caesar_decode(letter: u8, key: u8) -> u8 {
    ((letter + encode::SIZE) - key) % encode::SIZE
}

fn main() {
    let letter: u8 = 3;
    let key: u8 = 5;

    if encode::valid_value(letter) {
        let decoded: u8 = caesar_decode(letter, key % encode::SIZE);
        println!("{decoded}");
    } else {
        println!("-1");
    }
}
