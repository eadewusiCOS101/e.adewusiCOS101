use std::io;

fn main() {
    let mut stat:u16 = 1; 
    let mut input1 = String::new();
    let mut input2 = String::new();


    while stat != 0{

        println!("Are you experienced in this line of work? y or n.");
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let exp : &str = input1.trim();

        println!("How old are you?");
        io::stdin().read_line(&mut input2).expect("Invalid input.");
        let age:u16 = input2.trim().parse().expect("Invalid input.");


        if exp == "y" || exp == "Y"{
            let exp_status = "You are experienced.";
            println!("{}", exp_status);
            if exp_status == "You are experienced" && age >= 40{
                println!("Your incentive is NGN1,560,000.00");
            }

            else if exp_status == "You are experienced" && age >=30 && age < 40{
                println!("Your incentive is NGN1,480,000.00");
            }

            else{
                println!("Your incentive is NGN1,300,000.00");
            }
        }

        else if exp == "n" || exp == "N"{
            let exp_status = "You are inexperienced.";
            println!("{}", exp_status);
            println!("Your incentive is NGN100,000.00");
        }

        else{
            println!("Invalid input.");
            break;
        }

        stat -= 1;
    }
}
