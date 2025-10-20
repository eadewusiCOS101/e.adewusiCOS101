use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut input_1).expect("Invalid input.");
    let base:f32 = input_1.trim().parse().expect("Invalid input");

    println!("Enter height:");
    io::stdin().read_line(&mut input_2).expect("Invalid input.");
    let height:f32 = input_2.trim().parse().expect("Invalid input");

    if base > 0.0{
        let area:f32 = (base * height) / 2.0;
        println!("Area is {}", area);
    }


}
