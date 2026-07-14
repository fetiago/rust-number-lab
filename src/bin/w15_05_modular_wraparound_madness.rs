struct Number {
    high: u8,
    low: u8,
}

const MODULUS: Number = Number {
    high: 1,
    low: 109,
};

fn main() {
    let current = Number {
        high: 1,
        low: 34,
    };
    
    let step = Number {
        high: 0,
        low: 10,
    };

    // arithmetics start: current + step
    // low space available on `current`
    let space_low: u8 = u8::MAX - current.low;

    // low math
    let result_low: u8;
    let carry_low: u8;

    if step.low <= space_low {
        result_low = current.low + step.low;
        carry_low = 0;
    } else {
        result_low = step.low - space_low - 1;
        carry_low = 1;
    }

    // high math
    let mut result_high: u8 = current.high + step.high;

    if carry_low == 1 {
        result_high += 1;
    }
    // result = current + step
    
    // MODULUS arithmetics start
    // if result >= MODULUS
    let mut should_subtract: u8 = 0;

    if result_high > MODULUS.high {
        should_subtract = 1;
    }
    if result_high == MODULUS.high {
        if result_low >= MODULUS.low {
            should_subtract = 1;
        }
    }

    let final_high: u8;
    let final_low: u8;
    if should_subtract == 1 {
        // result - MODULUS
        if result_low >= MODULUS.low {
            final_low = result_low - MODULUS.low;
            final_high = result_high - MODULUS.high;
        } else {
            final_low = result_low + (u8::MAX - MODULUS.low) + 1;
            final_high = result_high - MODULUS.high - 1;
        }
    } else {
        final_low = result_low;
        final_high = result_high;
    }

    println!("({final_high},{final_low})");
}