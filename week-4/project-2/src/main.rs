use std::io;

fn main() {
    let mut stat:u16 = 1; 
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input_log = String::new();

    println!("Would you like to enable multiple login feature? y or n.");
    io::stdin().read_line(&mut input_log).expect("Invalid input.");
    let login:&str = input_log.trim();

    if login == "y" || login == "Y"{
        println!("Multi-user enabled.");
        println!("5 available uses.");
        stat = 5;
    }
    else if login == "n" || login == "N"{
        println!("Multi-user no longer available.");
        stat = 1;
    }
    else{
        println!("Invalid input.");
        println!("Single use enabled.");
    }


    while stat != 0{

        println!("Are you experienced in this line of work? y or n.");
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let exp : &str = input1.trim();

        if exp == "y" || exp == "Y"{
            println!("You are experienced.");
        }
        else if exp == "n" || exp == "N"{
            println!("You are inexperienced.");
        }
        else{
            println!("Invalid input.");
            break;
        }
        
        println!("How old are you?");
        io::stdin().read_line(&mut input2).expect("Invalid input.");
        let age:u16 = input2.trim().parse().expect("Invalid input.");

        if exp == "y" || exp == "Y" && age >= 40{
            println!("Your incentive is NGN1,560,000.00");
            
        } 
        else if exp == "y" || exp == "Y" && age >=30 && age < 40{
            println!("Your incentive is NGN1,480,000.00");
        }
        else if exp == "y" || exp == "Y" && age <28{
            println!("Your incentive is NGN1,300,000.00");
        }
        else{
            println!("Your incentive is NGN100,000.00");
        }

        stat -= 1;

    }
}
