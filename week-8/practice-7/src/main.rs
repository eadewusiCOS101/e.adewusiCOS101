fn main() {
    let datatype_tupule:(&str,f32,u8) = ("Rustc",3.142,100);
    println!("Tuple contents = {:?}",datatype_tupule);

    let no_datatype_tuple = ("Rust","fun",100);
    println!("Tuple contents = {:?}", no_datatype_tuple);

    println!("Value at index 0 = {}",datatype_tupule.0);
    println!("Value at index 1 = {}",datatype_tupule.1);   
    println!("Value at index 2 = {}",datatype_tupule.2);
}
