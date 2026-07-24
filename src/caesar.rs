pub const SIZE: u8 = 26; // ALPHABET SIZE

fn caesar_decode(letter: u8, key: u8) -> u8 {
    ((letter + encode::SIZE) - key) % encode::SIZE
}

pub fn valid_value(n: u8) -> bool {
    n < SIZE
}

pub fn caesar_encode(letter: u8, key: u8) -> u8 {
    (letter + key) % SIZE
}