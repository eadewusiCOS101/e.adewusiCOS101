fn main() {
    let mut num:i32 = 5;
    mutate_name_to_zero(&mut num);
    println!("the value of the number is {}", num);
}
fn mutate_name_to_zero(para_num: &mut i32){
    *para_num = *para_num * 0;
    println!("Param_num value is {}", para_num);
}