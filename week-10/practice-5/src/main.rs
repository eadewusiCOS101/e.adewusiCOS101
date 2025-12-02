fn main() {
    let x = vec![100,200,300];
    borrow_vector(&x);

    println!("Printing value from main() x[0] {}",x[0]);
}

fn borrow_vector(z:&Vec<i32>){
    println!("inside print vector function {:?}", z);
}