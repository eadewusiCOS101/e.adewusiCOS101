fn main() {
    let name = vec!["Gary","Sam","Victor","Josh","Marko","Harry","Benshaw","Mikari",""];
    let age = vec![13,14,15,21,17,19,22,16,];

    print!("\nAge allocation\n");
    
    for i in 0..age.len(){
        println!("{} is {} years old.",name[i],age[i]);
    }
}
