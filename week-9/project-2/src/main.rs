use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    name: String,
    matric: String,
    department: String,
    level: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    let mut input = String::new();

    println!("How many students do you want to enter?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let count: usize = input.trim().parse().expect("Enter a valid number");

    for i in 1..=count {
        println!("\n--- Student {} ---", i);

        input.clear();
        println!("Enter Student Name:");
        io::stdin().read_line(&mut input).expect("Failed");
        let name = input.trim().to_string();

        input.clear();
        println!("Enter Matric Number:");
        io::stdin().read_line(&mut input).expect("Failed");
        let matric = input.trim().to_string();

        input.clear();
        println!("Enter Department:");
        io::stdin().read_line(&mut input).expect("Failed");
        let department = input.trim().to_string();

        // Level
        input.clear();
        println!("Enter Level:");
        io::stdin().read_line(&mut input).expect("Failed");
        let level: u32 = input.trim().parse().expect("Enter a number");

        students.push(Student {
            name,
            matric,
            department,
            level,
        });
    }

    println!("\n==============================");
    println!("        PAU SMIS TABLE        ");
    println!("==============================");
    println!("No | Student Name | Matric Number | Department | Level");

    for (i, s) in students.iter().enumerate() {
        println!(
            "{}  | {} | {} | {} | {}",
            i + 1,
            s.name,
            s.matric,
            s.department,
            s.level
        );
    }

    // ---------- Save to File ----------
    let mut file = File::create("students.txt").expect("Could not create file");

    writeln!(file, "PAU SMIS STUDENT RECORDS").unwrap();
    writeln!(file, "No | Student Name | Matric Number | Department | Level").unwrap();

    for (i, s) in students.iter().enumerate() {
        writeln!(
            file,
            "{}  | {} | {} | {} | {}",
            i + 1,
            s.name,
            s.matric,
            s.department,
            s.level
        ).unwrap();
    }

    println!("\nStudent records saved to students.txt!");
}
