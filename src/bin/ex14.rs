// circle

const PI: f32 = 3.14159;

fn main() {
    let r: f32 = 5.0;

    let circumference = 2.0 * PI * r;
    let area = PI * r * r;

    println!("circumference: {}\narea: {}", circumference, area);
}
