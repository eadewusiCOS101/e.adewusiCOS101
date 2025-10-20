use std::io;

fn main() {
    println!("Enter lower bound.");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid bound.");
    let lower_bound:i32 = input1.trim().parse().expect("Invalid bound.");

    println!("Enter upper bound.");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid bound.");
    let upper_bound:i32 = input2.trim().parse().expect("Invalid bound.");

    for x in lower_bound..upper_bound {
        println!("Count is {}", x)
    }


}
