use std::io;

struct Laptop{
    name:String,
    price:f64,
    quantity:f64
}

fn get_info(prompt:String) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read Input");
    let result:String = input.trim().to_string();
    return result
}

fn get_info_as_float(prompt:String) -> f64{
    println!("{}",prompt);
    let mut response:f64 = 0.0;
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read Input");
        let result:String = input.trim().to_string();
        if let Ok(num) = result.parse::<f64>(){
            response = num;
            break 
        }
        else{
            println!("Invalid input");
            println!("{}",prompt);
        }
    }
    return response
}

fn main() {
    loop{
        let hp = Laptop{
            name:String::from("HP"),
            price:650000.0,
            quantity:10.0,
        };

        let ibm = Laptop{
                name:String::from("IBM"),
                price:755000.0,
                quantity:6.0,
            };

        let toshiba = Laptop{
                name:String::from("Toshiba"),
                price:550000.0,
                quantity:10.0,
            };

        let dell = Laptop{
                name:String::from("Dell"),
                price:850000.0,
                quantity:4.0,
            };
        let laptop_vector = vec!["HP","IBM","TOSHIBA","DELL"];

        let name_order:String = loop{
            let name_order_input = get_info("What laptop would you like to buy (Make sure the name is all caps)\n".to_string());
            let noil:&str = &name_order_input;
            if laptop_vector.contains(&noil){
                println!("Order for {} laptop initialized.", name_order_input);
                break name_order_input.to_string();
                }
            else {
                println!("Invalid, Enter one of these {:?}", laptop_vector);
            }
        };

        let mut _name_to_use:&str = "";
        let mut _price_to_use:f64 = 0.0;
    
        let _quantity_order:f64 = loop{
            let _quantity_order_input:String = get_info_as_float("\nWhat number of units?".to_string()).to_string();
            let _qoil:f64 = _quantity_order_input.parse().expect("Failed to read Input");

            if _quantity_order_input.contains('.'){
                println!("Invalid, impossible to buy a fraction of a laptop.");
            }
            else if _qoil < 0.0{
                println!("Invalid, cannot order a negetive quantity of laptops.");
            }
            else if name_order == "HP"{
                if _qoil > hp.quantity{
                    println!("Invalid, there are 10 units of HP laptops to buy.");
                }
                else{
                    _name_to_use = &hp.name;
                    _price_to_use = hp.price;
                    break _qoil;
                }
            }
            else if name_order == "IBM"{
                if _qoil > ibm.quantity{
                    println!("Invalid, there are 6 units of IBM laptops to buy.");
                }
                else{
                    _name_to_use = &ibm.name;
                    _price_to_use = ibm.price; 
                    break _qoil;
                }
            }
            else if name_order == "TOSHIBA"{
                if _qoil > toshiba.quantity{
                    println!("Invalid, there are 10 units of Toshiba laptops to buy.");
                }
                else{
                    _name_to_use = &toshiba.name;
                    _price_to_use = toshiba.price;
                    break _qoil;
                }
            }
            else if name_order == "DELL"{
                if _qoil > dell.quantity{
                    println!("Invalid, there are 4 units of Dell laptops to buy.");
                }
                else{
                    _name_to_use = &dell.name;
                    _price_to_use = dell.price;
                    break _qoil;
                }
            }
            else{
                break _qoil;
            }
        };

        let mut order:f64 = 0.0;
        let mut discount:&str = "";

        let calc:f64 = _quantity_order * _price_to_use;
        if calc > 1400000.0{
            order = calc - (0.07 * calc);
            discount = "(7% discount)"
        }
        else{
            order = calc;
        }
        println!("Your bill for {} {} laptops is ₦{:.2}{}\n", _quantity_order, _name_to_use, order, discount);

        let mut shutdown:&str = "";

        let mut retry:String =loop{
            let retry_input:String = get_info("Would you like to make another order? y or n\n".to_string());
            if retry_input == "y" || retry_input == "Y"{
                println!("Proceed to make another order.");
                break retry_input;
            }
            else if retry_input =="n" || retry_input == "N"{
                println!("Program shutdown.");
                shutdown = "kill";
                break retry_input;
            } 
            else{
                println!("Invalid, write y or n only.");
            }
        };
        if shutdown == "kill"{
            break;
        }
    }
}
