use std::io;

fn main() {
    println!("Are you Experienced? y or n.");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid input.");
    let exp:char = input1.trim().parse().expect("Invalid input.");

    println!("Enter your age.");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid input.");
    let age:u32 = input2.trim().parse().expect("Invalid input.");

    if exp == 'y' && age >= 40{
        println!("Your incentive is NGN1,560,000");
    }
    else if exp == 'y' && age >= 30 && age < 40{
        println!("Your incentive is NGN1,480,000");
    } 
    else if exp == 'y' && age < 28{
        println!("Your incentive is NGN1,300,000");
    }
    else{
        println!("Your incentive is NGN100,000");
    }
}
