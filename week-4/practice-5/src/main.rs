use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your height (in centimeters)");
    io::stdin().read_line(&mut input).expect("Not valid.");
    let height:f32 = input.trim().parse().expect("Not valid.");

    if height >= 150.0 && height <= 170.0{
        println!("You have average height.");
    }
    else if height > 170.0 && height <= 195.0{
        println!("You are tall.");
    }
    else if height < 150.0 && height >100.0{
        println!("You are short 😔");
    }
    else{
        println!("Abnormal height.");
    }
}