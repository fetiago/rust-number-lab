// right triangle's area

fn main() {
    let base: f32 = 12.0;
    let height: f32 = 24.0;

    let area = (base * height) / 2.0;
    let perimeter = base + height + (base * base + height * height).sqrt();
    println!("area: {:.2}\nperimeter: {:.2}", area, perimeter);
}
