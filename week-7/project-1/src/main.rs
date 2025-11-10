use std::io;

fn trapezium(){
    let mut input_height = String::new();
    println!("Input height");
    io::stdin().read_line(&mut input_height).expect("Invalid input");
    let mut h:f32 = input_height.trim().parse().expect("Invalid input");

    let mut input_a = String::new();
    println!("Input first parallel side");
    io::stdin().read_line(&mut input_a).expect("Invalid input");
    let mut a:f32 = input_a.trim().parse().expect("Invalid input");

    let mut input_b = String::new();
    println!("Input second parallel side");
    io::stdin().read_line(&mut input_b).expect("Invalid input");
    let mut b:f32 = input_b.trim().parse().expect("Invalid input");

    let mut area:f32 = ((a + b) * h) * 0.5;

    println!("The area of a trapezium with parallel sides {} and {} and height {} is {:.3} \n", a, b, h, area);
}

fn rhombus(){

    let mut input_d1 = String::new();
    println!("Input first diagonal");
    io::stdin().read_line(&mut input_d1).expect("Invalid input");
    let mut a:f32 = input_d1.trim().parse().expect("Invalid input");

    let mut input_d2 = String::new();
    println!("Input second diagonal");
    io::stdin().read_line(&mut input_d2).expect("Invalid input");
    let mut b:f32 = input_d2.trim().parse().expect("Invalid input");

    let mut area:f32 = a * b * 0.5;

    println!("The area of a rhombus with diagonals {} and {} is {:.3} \n", a, b, area);
}

fn parallelogram(){

    let mut input_base = String::new();
    println!("Input base");
    io::stdin().read_line(&mut input_base).expect("Invalid input");
    let mut b:f32 = input_base.trim().parse().expect("Invalid input");

    let mut input_height = String::new();
    println!("Input altitude");
    io::stdin().read_line(&mut input_height).expect("Invalid input");
    let mut h:f32 = input_height.trim().parse().expect("Invalid input");

    let mut area:f32 = h * b ;

    println!("The area of a parallelogram with base {} and altitude {} is {:.3} \n", b, h, area);
}

fn cube(){

    let mut input_l = String::new();
    println!("Input Length of side");
    io::stdin().read_line(&mut input_l).expect("Invalid input");
    let mut l:f32 = input_l.trim().parse().expect("Invalid input");

    let mut area:f32 = 6.0 * l.powf(2.0) ;

    println!("The surface area of a cube with length {} is {:.3} \n", l, area);
}

fn cylinder(){

    let mut input_radius = String::new();
    println!("Input radius");
    io::stdin().read_line(&mut input_radius).expect("Invalid input");
    let mut r:f32 = input_radius.trim().parse().expect("Invalid input");

    let mut input_height = String::new();
    println!("Input height");
    io::stdin().read_line(&mut input_height).expect("Invalid input");
    let mut h:f32 = input_height.trim().parse().expect("Invalid input");

    let mut volume:f32 = (22.0 / 7.0) * h * (r.powf(2.0)) ;

    println!("The volume of a cylinder with base radius {} and height {} is {:.3} \n", r, h, volume);
}


fn main() {
    println!("Welcome to the dimensional calculator \n
Use it to calculate various areas and volumes. \n
\n");
    loop{
        let mut input_op = String::new();
        println!("Code   Operation \n
T      Area of a trapezium \n
R      Area of a rhombus \n
P      Area of a parallelogram \n
C      Surface area of a cube \n
Y      Volume of a cylinder \n
(Use Q to quit)");
        io::stdin().read_line(&mut input_op).expect("Invalid input");
        let mut op:String = input_op.trim().to_uppercase();
        if op == "T"{
            trapezium();
        }
        else if op == "R"{
            rhombus();
        }
        else if op == "P"{
            parallelogram();
        }
        else if op == "C"{
            cube();
        }
        else if op == "C"{
            cube();
        }
        else if op == "Y"{
            cylinder();
        }
        else if op== "Q"{
            return;
        }
        else{
            println!("Invalid input, retry");
        }

    }
  
}
