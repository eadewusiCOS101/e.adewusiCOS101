use std::io;

fn main() {
    let jobs:Vec<&str> = vec!["Office administrator","Academic","Lawyer","Teacher"];
    println!("Jobs available are: {:?}",jobs);
    println!("Enter Job:");
    let mut job_input = String::new();
    io::stdin().read_line(&mut job_input).expect("Invalid input");
    let job:&str = job_input.trim();

    let office_admin:Vec<&str> = vec!["Intern","Administrator","Senior Administrator","Office manager","Director","Cheif Executive officer"];
    let academic:Vec<&str> = vec!["","Reasearch Assistant","PhD Candidate","Post-Doc Researcher","Senior lecturer","Dean"];
    let lawyer:Vec<&str> = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let teacher:Vec<&str> = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Teacher","Principal"];

    let job_valid = job.to_lowercase();

    if job_valid == jobs[0].to_lowercase(){
        println!("{:?}",office_admin);
        aps_determination(office_admin);
    }
    else if job_valid == jobs[1].to_lowercase(){
        println!("{:?}",academic);
        aps_determination(academic);
    }
    else if job_valid == jobs[2].to_lowercase(){
        println!("{:?}",lawyer);
        aps_determination(lawyer);
    }   
    else if job_valid == jobs[3].to_lowercase(){
        println!("{:?}",teacher);
        aps_determination(teacher);
    }
}

fn aps_determination(x:Vec<&str>){
    println!("Identify Job title:");
    let mut title_input = String::new();
    io::stdin().read_line(&mut title_input).expect("Invalid input");
    let title:&str = title_input.trim();

    println!("Clarify years of expertise.");
    let mut year_input = String::new();
    io::stdin().read_line(&mut year_input).expect("Invalid input");
    let years:u16 = year_input.trim().parse().expect("Invalid input");

    let title_valid = title.to_lowercase();

    if title_valid == x[0].to_lowercase(){
        println!("Staff level: APS 1-2");
    }
    else if title_valid == x[1].to_lowercase(){
        println!("Staff level: APS 3-5");
    }
    else if title_valid == x[2].to_lowercase(){
        println!("Staff level: APS 5-8");
    }
    else if title_valid == x[3].to_lowercase(){
        println!("Staff level: EL1 8-10");
    }
    else if title_valid == x[4].to_lowercase(){
        println!("Staff level: EL2 10-13");
    }
    else if title_valid == x[5].to_lowercase(){
        println!("Staff level: SES");
    }
    else{
        println!("Invalid.");
    }
}
