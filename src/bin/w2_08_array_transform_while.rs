#[path = "w2_04_caesar_encode_fn.rs"]
mod encode;

fn main() {
    let letters: [u8; 5] = [7, 4, 11, 11, 14];
    let key: u8 = 3;
    let mut encoded: [u8; 5] = [0, 0, 0, 0, 0];

    let mut valid: bool = true;
    let mut i: usize = 0;
    while i < letters.len() && valid {
        valid = encode::valid_value(letters[i]);
        i += 1;
    }
    // encoding
    if valid {
        i = 0;
        while i < letters.len() {
            encoded[i] = encode::caesar_encode(letters[i], key % encode::SIZE);
            i += 1;
        }
    } else {
        println!("-1");
    }
    // printing  
    if valid {
        i = 0;
        while i < letters.len() {
            println!("{}", encoded[i]);
            i += 1;
        }
    }
}
