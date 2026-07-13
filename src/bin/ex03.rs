// 24h clock

fn main() {
    let current_time: i32 = 22;
    let remaining_time: i32 = 8;

    let estimated_time: i32 = (current_time + remaining_time) % 24;
    println!("estimated time is {}h.", estimated_time);
}
