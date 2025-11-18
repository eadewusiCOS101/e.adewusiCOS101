fn main() {
    let b:(i32,bool,f64) = (30,true,4.9);
    printing(b);
}
fn printing(x:(i32,bool,f64)){
    println!("Inside print method");
    let (age,is_male,cgpa) = x;
    println!("Age is {}, is male? {}, CGPA is {}",age,is_male,cgpa);
}
