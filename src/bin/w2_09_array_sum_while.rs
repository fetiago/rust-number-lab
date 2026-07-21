fn main() {
    let arr: [u8; 5] = [10, 20, 30, 40, 50];
    let mut sum: u32 = 0;
    let mut i: usize = 0;
    while i < arr.len() {
        sum += arr[i] as u32;
        i += 1;
    }
    println!("{}", sum);
}
