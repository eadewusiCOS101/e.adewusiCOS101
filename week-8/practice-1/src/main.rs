fn main() {
    let v : Vec<i64> = Vec::new();
    println!("\n The length of Vec::new(): {}", v.len());

    let v = vec!["Alexis" , "Garvand" , "Sornata" , "Cranshaw" , "Baro"];
    println!("The length of the vec macro is {}", v.len());
}
