fn main() {
    let b:(i32,bool,f64) = (110,true,10.9);
    printing(b);
}
fn printing(x:(i32,bool,f64)){
    println!("Inside print method");
    println!("{:?}",x);
}