use std::io;

fn main() {
    println!("Provided that the form of quadratic equation is ax^2 + bx + c");
    println!("Input a:");
    let mut a_var = String::new();
    io::stdin().read_line(&mut a_var).expect("Invalid.");
    let a:f32 = a_var.trim().parse().expect("Invalid.");

    println!("Input b:");
    let mut b_var = String::new();
    io::stdin().read_line(&mut b_var).expect("Invalid.");
    let b:f32 = b_var.trim().parse().expect("Invalid."); 

    println!("Input c:");
    let mut c_var = String::new();
    io::stdin().read_line(&mut c_var).expect("Invalid.");
    let c:f32 = c_var.trim().parse().expect("Invalid.");  

    let d:f32 = (b * b) - (4.0 * a * c);// Calculating the discriminant (d).
    if d >= 0.0{
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("The roots of the equation are {} and {}", root1, root2);
        if root1 == root2{
            println!("Therefore, the root of the equation is {} twice.", root1);
        }
    }

    else{
        println!("The roots of this equation are Imaginary, hence they cannot be found.");
    }
}
