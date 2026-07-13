// caesar cipher w/ numbers

const ALPHABET_SIZE: i32 = 26;

fn main() {
    let letter: i32 = 24;
    let key: i32 = 5;

    // code
    let encoded: i32 = (letter + key) % ALPHABET_SIZE;
    println!("the code is {}", encoded);

    // decode
    let decoded: i32 = (encoded + ALPHABET_SIZE - key) % ALPHABET_SIZE;
    println!("the letter is {}", decoded);
}
