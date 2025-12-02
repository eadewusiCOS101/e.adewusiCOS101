struct Employee{
    name:String,
    company:String,
    age:i32
}

fn main() {
    let emp1 = Employee{
        company:String::from("Ernst & young"),
        name:String::from("Garret Sonfero"),
        age:25
    };
    println!("Name: {} \n
Company: {} \n
Age: {}", emp1.name, emp1.company, emp1.age);
}
