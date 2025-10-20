use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name.");
    io::stdin().read_line(&mut input1).expect("Wait...what?");

    println!("Enter your age.");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let age:u32 = input2.trim().parse().expect("Aint no way you messed up ur age");

    if age >= 18{
        println!("You are invited to the party!");
    }
    else{
        println!("You are not of the appropriate age to join, sorry.");
    }
}
