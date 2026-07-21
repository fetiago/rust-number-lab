#[path = "w2_04_caesar_encode_fn.rs"]
mod encode;

fn main() {
    let letter: [u8; 5] = [7, 4, 11, 11, 14];
    let key: u8 = 3;
    let mut encoded: [u8; 5] = [0, 0, 0, 0, 0];

    let mut valid: bool = true;
    let mut i: usize = 0;
    while i < letter.len() && valid {
        valid = encode::valid_value(letter[i]);
        i += 1;
    }
    if valid {
        i = 0;
        while i < letter.len() {
            encoded[i] = encode::caesar_encode(letter[i], key % encode::SIZE);
            println!("{}", encoded[i]);
            i += 1;
        }
    } else {
        println!("-1");
    }
}
