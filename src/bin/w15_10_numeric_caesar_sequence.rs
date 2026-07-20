const ALPHABET_SIZE: u8 = 26;

fn main() {
    let array: [u8; 5] = [7, 4, 11, 11, 14];
    let key: u8 = 3;

    let encoded: [u8; 5] = [
        (array[0] + key) % ALPHABET_SIZE,
        (array[1] + key) % ALPHABET_SIZE,
        (array[2] + key) % ALPHABET_SIZE,
        (array[3] + key) % ALPHABET_SIZE,
        (array[4] + key) % ALPHABET_SIZE,
    ];

    println!(
        "letter {}: {}\nletter {}: {}\nletter {}: {}\nletter {}: {}\nletter {}: {}\n",
        1, encoded[0], 2, encoded[1], 3, encoded[2], 4, encoded[3], 5, encoded[4]
    );
}
