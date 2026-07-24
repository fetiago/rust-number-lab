use rust_number_lab::caesar;

fn decoder(x: [u8; 5], k: u8) -> [u8; 5] {
    let mut temp: [u8; 5] = [0; 5];
    let mut i: usize = 0;
    let mut checked_chars: bool = true;
    while i < x.len() {
        if !caesar::valid_value(x[i]) {
            checked_chars = caesar::valid_value(x[i]);
            i = x.len();
        }
        i += 1;
    }
    if checked_chars {
        i = 0;
        while i < x.len() {
            temp[i] = caesar::caesar_decode(x[i], k);
            i += 1;
        }
        temp
    } else {
        println!("err: invalid value(s), returning 0 array");
        temp
    }
}

fn encoder(x: [u8; 5], k: u8) -> [u8; 5] {
    let mut temp: [u8; 5] = [0; 5];
    let mut i: usize = 0;
    let mut checked_chars: bool = true;
    while i < x.len() {
        if !caesar::valid_value(x[i]) {
            checked_chars = caesar::valid_value(x[i]);
            i = x.len();
        }
        i += 1;
    }
    if checked_chars {
        i = 0;
        while i < x.len() {
            temp[i] = caesar::caesar_encode(x[i], k);
            i += 1;
        }
        temp
    } else {
        println!("err: invalid value(s), returning 0 array");
        temp
    }
}

fn main() {
    let msg_to_encode: [u8; 5] = [7, 4, 11, 11, 14];
    let key: u8 = 5;

    let encoded_msg: [u8; 5] = encoder(msg_to_encode, key);
    let decoded_msg: [u8; 5] = decoder(encoded_msg, key);

    let mut i: usize = 0;
    while i < encoded_msg.len() {
        println!("{}", encoded_msg[i]);
        i += 1;
    }
    i = 0;
    while i < decoded_msg.len() {
        println!("{}", decoded_msg[i]);
        i += 1;
    }
}