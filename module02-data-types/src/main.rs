fn main() {
    
    // Rust Data types are classified into two
    // 1 --Scalar Data Type(Integer, Float, Boolean, Character)
    // Integer
    let age: i32 = 25;

    // Floating point
    let salary: f64 = 5000.75;

    // Boolean
    let is_active: bool = true;

    // Character
    let grade: char = 'A';

    println!("Age: {}", age);
    println!("Salary: {}", salary);
    println!("Active: {}", is_active);
    println!("Grade: {}", grade);
    
    // 2-- Compound Data type ( Tuple and Array)
    // Tuple
    let person = ("Steven", 25, true);

    println!("Name: {}", person.0);
    println!("Age: {}", person.1);

    // Array
    let numbers = [10, 20, 30, 40];

    println!("First Number: {}", numbers[0]);
}