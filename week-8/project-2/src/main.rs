use std::io;

fn main() {
    println!("=== EY Nigeria - Developer Experience Checker ===");
    println!("Enter candidate details (name and years of experience)\n");
    
    // Vector to store candidates as tuples (name, experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();
    
    loop {
        println!("Enter candidate name (or 'done' to finish):");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim().to_string();
        
        if name.to_lowercase() == "done" {
            break;
        }
        
        println!("Enter years of experience for {}:", name);
        let mut experience = String::new();
        io::stdin().read_line(&mut experience).expect("Failed to read input");
        
        let experience: u32 = match experience.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.");
                continue;
            }
        };
        
        // Add candidate to vector as a tuple
        candidates.push((name, experience));
        println!("Candidate added successfully!\n");
    }
    
    // Display all candidates
    println!("\n=== All Candidates ===");
    for (index, (name, exp)) in candidates.iter().enumerate() {
        println!("{}. {} - {} years", index + 1, name, exp);
    }
    
    // Find candidate with maximum experience
    if candidates.is_empty() {
        println!("\nNo candidates entered!");
        return;
    }
    
    let mut max_experience = 0;
    let mut top_candidate = &candidates[0];
    
    // Iterate through vector to find maximum
    for candidate in &candidates {
        if candidate.1 > max_experience {
            max_experience = candidate.1;
            top_candidate = candidate;
        }
    }
    
    // Display results
    println!("\n=== Interview Results ===");
    println!("🏆 Top Candidate: {}", top_candidate.0);
    println!("💪 Years of Experience: {}", top_candidate.1);
    println!("🎉 Congratulations! You're hired by EY Nigeria!");
}
