const SIZE: u8 = 26; // ALPHABET SIZE

fn valid_value(n: u8) -> bool { n < SIZE }

fn caesar_encode(letter: u8, key: u8) -> u8 { (letter + key) % SIZE }

fn main() {
    let letter: u8 = 24;
    let key: u8 = 5;

    if valid_value(letter) {
        let encoded: u8 = caesar_encode(letter, key % SIZE);
        println!("{encoded}");
    } else {
        println!("-1");
    }
}
