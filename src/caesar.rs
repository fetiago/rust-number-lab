pub const SIZE: u8 = 26; // ALPHABET SIZE

pub fn valid_value(n: u8) -> bool {
    n < SIZE
}

pub fn caesar_encode(letter: u8, key: u8) -> u8 {
    (letter + key) % SIZE
}

pub fn caesar_decode(letter: u8, key: u8) -> u8 {
    ((letter + SIZE) - key) % SIZE
}