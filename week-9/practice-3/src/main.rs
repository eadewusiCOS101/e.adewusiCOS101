use std::fs;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut file_import = std::fs::File::create("data.txt").expect("Creation failed");
    file_import.write_all("This file has the following data to be destroyed. Back up the data safely.\n

Zulu Echo Alpha Charlie Tango Delta Bravo\n".as_bytes()).expect("Data extract filed");
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);

    fs::remove_file("data.txt").expect("couldn't remove file");
    println!("File remved successfully");
}
