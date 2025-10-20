use std::io;

fn main() {
    println!("\nStudent Information Management System.");

    println!("\nPlease enter your name.");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("failed to read input!");
    println!("So, your name is {}", name);

    println!("Alright {}, how about your age?", name);
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");//This reads what is being written in the input.
    let age:u32 = age.trim().parse().expect("Input is not valid");
    println!("You are {} years old.", age);
}
