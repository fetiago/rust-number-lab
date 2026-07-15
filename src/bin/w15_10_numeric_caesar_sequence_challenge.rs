const ALPHABET_SIZE: u8 = 26;

fn main() {
    let array: [u8; 5] = [7, 4, 11, 11, 14];
    let key: u8 = 3;

    let encoded: [u8; 5] = {
        let mut index: usize = 0;
        let mut temp: [u8; 5] = [0; 5];
        let safe_key: u8 = key % ALPHABET_SIZE;
        while index < array.len() {
            if array[index] < ALPHABET_SIZE {
                temp[index] = (array[index] + safe_key) % ALPHABET_SIZE;
            }
            index += 1;
        }
        temp
    };

    println!(
        "letter {}: {}\nletter {}: {}\nletter {}: {}\nletter {}: {}\nletter {}: {}\n",
        1, encoded[0],
        2, encoded[1],
        3, encoded[2],
        4, encoded[3],
        5, encoded[4]
    );
}
