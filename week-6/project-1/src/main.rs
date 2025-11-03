use std::io;

fn main() {
    println!("Menu \n
Code      Food                         Price (₦) \n
P         Pounded yam/Edinkaiko soup   3,200 \n
F         Fried rice and chicken       3,000 \n 
A         Amala and ewedu              2,500 \n 
E         Eba and egusi soup           2,000 \n 
W         White rice and stew          2,500 \n
(Use Q to quit) \n");
    loop{
        let mut input_code = String::new();
        let mut inputq = String::new();

        println!("Input food code");
        io::stdin().read_line(&mut input_code).expect("Invalid code");
        let code:String = input_code.trim().to_uppercase();
        if (code != "P") && (code != "F") && (code != "A") && (code != "E") && (code != "W") && (code != "Q"){
            println!("Invalid code, program shutdown.");
            return;
        }
        else if code == "Q"{
            println!("Program shutdown.");
            return;
        }
        else{
            
        }

        println!("How many?");
        io::stdin().read_line(&mut inputq).expect("Invalid quantity");
        let quantity:f64 = inputq.trim().parse().expect("Invalid quantity");

        if code == "P"{
            let food = "Pounded yam/Edinkaiko soup";
            let order = 3200.00 * quantity;
            if order > 10000.00{
                let final_order = order - (order * 0.05);
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", final_order, quantity, food);
            }
            else{
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", order, quantity, food);
            }
        }

        else if code == "F"{
            let food = "Fried rice and chicken";
            let order = 3000.00 * quantity;
            if order > 10000.00{
                let final_order = order - (order * 0.05);
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", final_order, quantity, food);
            }
            else{
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", order, quantity, food);
            }
        }

        else if code == "A"{
            let food = "Amala and ewedu";
            let order = 2500.00 * quantity;
            if order > 10000.00{
                let final_order = order - (order * 0.05);
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", final_order, quantity, food);
            }
            else{
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", order, quantity, food);
            }
        }

        else if code == "E"{
            let food = "Eba and egusi soup";
            let order = 2000.00 * quantity;
            if order > 10000.00{
                let final_order = order - (order * 0.05);
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", final_order, quantity, food);
            }
            else{
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", order, quantity, food);
            }
        }

        else if code == "W"{
            let food = "White rice and stew";
            let order = 2500.00 * quantity;
            if order > 10000.00{
                let final_order = order - (order * 0.05);
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", final_order, quantity, food);
            }
            else{
                println!("Your tab is ₦{:.2} for {} unit(s) of {}. \n", order, quantity, food);
            }
        }

        else{
            println!("This order is invalid... and I am going to fire someone.");
        }

    }

}
