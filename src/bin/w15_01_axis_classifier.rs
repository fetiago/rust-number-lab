fn main() {
    let x: i32 = 0;
    let y: i32 = 0;

    if x==0 && y==0 {
        println!("origin");
    } 
    else if y==0 {
        println!("x axis");
    }
    else if x==0 {
        println!("y axis");
    }
    else if x>0 && y>0 {
        println!("quadrant 1");
    }
    else if x<0 && y>0 {
        println!("quadrant 2");
    }
    else if x<0 && y<0 {
        println!("quadrant 3");
    }
    else {
        println!("quadrant 4");
    }
}
