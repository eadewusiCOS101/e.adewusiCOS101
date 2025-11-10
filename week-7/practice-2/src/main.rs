use std::io;

fn checker(){
    let mut input = String::new();
    println!("Enter a character.");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let ch:char = input.trim().parse().expect("Invalid input");

    if ch >= '0' && ch <= '9'{
        println!("This is a digit");
    }
    else{
        println!("This is not a digit");
    }
} 

fn main() {
    println!("Welcome, this software shecks if the character variable is a digit.");
    checker();
}
