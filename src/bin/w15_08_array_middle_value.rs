fn main() {
    let array: [u8; 5] = [10, 21, 31, 41, 50];
    let middle: usize = array.len() / 2; // for odd arrays
    
    println!("middle_index: {}\nmiddle_element: {}", middle, array[middle]);
}
