use std::fs;
use std::io::Write;

fn main() {
    let data_table = "\n
Lager        Stout         Non-Alcoholic\n
33 Export    Legend        Maltina
Desperados   Turbo king    Amstel Malta
Goldberg     Williams      Malta Gold
Guilder                    Fayrouz
Heineken
Gulder.";

    let mut file = std::fs::File::create("Nigerian Breweries Plc. high quality breweries.txt").expect("Creation failed");
    file.write_all(data_table.as_bytes()).expect("Failed to write the table.");
    println!("File successfully created and stored.");
}
