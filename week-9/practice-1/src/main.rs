use std::io::Write;

fn main() {
    
    let announce = "week 9 - Rust file input and output\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("Creation failed");
    file.write_all("Welcome to Rust programming\n"
        .as_bytes()).expect("Write failed");
    file.write_all(announce.as_bytes()).expect("Write failed");
    file.write_all(dept.as_bytes()).expect("Write failed");
    println!("\nData written into a file. ");

}
