fn main() {
    let array: [u8; 5] = [10, 21, 31, 41, 50];
    let first: usize = 0;
    let last: usize = array.len() - 1; // array.len() returns usize type
    
    println!(
        "first: {}\nlast: {}\nsum: {}",
        array[first],
        array[last],
        array[first] + array[last]
    );
}
