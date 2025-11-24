fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Cannot open File.");
    file.write_all("\nHello class".as_bytes()).expect("Writ failed");
    file.write_all("\n This is the appendage to the document.".as_bytes()).expect("Write failed");
    println!("File append succeded");
}
