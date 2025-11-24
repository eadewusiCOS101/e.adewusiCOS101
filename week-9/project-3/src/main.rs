use std::fs::File;
use std::io::Write;

fn main(){
    let commisioner_name = vec![
    "Aigbogun Alamba Daudu      ",
    "Murutala Afeez Bendu       ",
    "Okorocha Calistus Ogbonga  ",
    "Adewale Jimoh Akanbi       ",
    "Osazuwa Faith Eiteye       "];

    let commisioner_ministry = vec![
    "Internal Affairs   ",
    "Justice            ",
    "Defense            ",
    "Power and Steel    ",
    "Petroleum          "];

    let commisioner_gpz = vec![
    "South West ",
    "North East ",
    "South South",
    "South West ",
    "South East "];

    let mut file = File::create("Commisioner info.txt").expect("Unable to create file.");
    file.write_all("All Commisioner information extracted from the backup.\n
 Name                           Ministry      Geopolictical Zone.\n".as_bytes()).expect("Could not complete");

    for c in 0..commisioner_name.len(){
        let line = format!("{}{}{}\n",commisioner_name[c], commisioner_ministry[c],commisioner_gpz[c]);
        file.write_all(line.as_bytes()).expect("Error occured");
    }
    println!("Information Written successfully into the file!");
    
}