use std::fs::File;
use std::io::Write;

fn main(){
    let data = vec![
    ("Aigbogun Alamba Daudu","Internal Affairs","South West"),
    ("Murutala Afeez Bendu","Justice","North East"),
    ("Okorocha Calistus Ogbonga","Defense","South South"),
    ("Adewale Jimoh Akanbi","Power and Steel","South West"),
    ("Osazuwa Faith Eiteye","Petroleum","South East")];

    let mut file = File::create("Commisioner info.txt").expect("Unable to create file.");
    file.write_all("All Commisioner information extracted from the backup.\n
Order of information mention: Name, Ministry, Geopolictical Zone.\n".as_bytes()).expect("Could not complete");

    for commisioner in 0..data.len(){
        let line = format!("{:?}\n",data[commisioner]);
        file.write_all(line.as_bytes()).expect("Error occured");
    }
    println!("Information Written successfully into the file!");
}