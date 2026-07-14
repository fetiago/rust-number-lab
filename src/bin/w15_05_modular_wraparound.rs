
fn main() {
    let current: u8 = 250;
    let step: u8 = 10;
    
    // u8 := 0 ~ 255
    // there's two concepts
    // 1) wrapping_add()
    let result1: u8 = current.wrapping_add(step); // result = 4
    
    // 2) u8::MAX == 255 as u8 variable
    let remaining_until_max: u8 = u8::MAX - current; // 255 - current
    let result2: u8 = if step > remaining_until_max { 
        step - remaining_until_max - 1
    } else { 
        current + step
    };

    if result1 == result2 {
        println!("it works!");
    }
}
