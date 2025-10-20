use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();

    println!(" Enter the first side of the triangle.");
    io::stdin().read_line(&mut input_1).expect("Invalid...");
    let a:f32 = input_1.trim().parse().expect("Invalid..."); 

    println!(" Enter the second side of the triangle.");
    io::stdin().read_line(&mut input_2).expect("Invalid...");
    let b:f32 = input_2.trim().parse().expect("Invalid..."); 

    println!(" Enter the first side of the triangle.");
    io::stdin().read_line(&mut input_3).expect("Invalid...");
    let c:f32 = input_3.trim().parse().expect("Invalid..."); 

    let s:f32 = (a + b + c) / 2.0;

    let mut area:f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();

    println!("the area of this triangle is {}", area,);
}
