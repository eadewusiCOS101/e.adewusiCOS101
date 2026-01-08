fn main() {
    let data = ["Ade", "Ola", "Joye", "Adam", "Yemi", "Sam", "Tola"];
    pass_me(&data[4..]);
}

fn pass_me(use_data:&[&str]){
    println!("use_data length is {}",use_data.len());
    println!("{:?}",use_data);
}
