fn main() {

    println!("========== OWNERSHIP ==========");

    let name1 = String::from("Steven");

    // Ownership moves from name1 to name2
    let name2 = name1;

    println!("New Owner: {}", name2);

    // println!("{}", name1);
    // Error: name1 no longer owns the value


    println!("\n========== CLONING ==========");

    let city1 = String::from("Lagos");

    // Deep copy
    let city2 = city1.clone();

    println!("City 1: {}", city1);
    println!("City 2: {}", city2);


    println!("\n========== OWNERSHIP IN FUNCTIONS ==========");

    let message = String::from("Rust Ownership");

    take_ownership(message);

    // println!("{}", message);
    // Error: ownership moved into function


    println!("\n========== BORROWING ==========");

    let course = String::from("Rust Programming");

    borrow_value(&course);

    // course is still valid
    println!("Main Function: {}", course);


    println!("\n========== MUTABLE BORROWING ==========");

    let mut username = String::from("Steven");

    change_username(&mut username);

    println!("Updated Username: {}", username);
}


// Function takes ownership
fn take_ownership(text: String) {

    println!("Inside Function: {}", text);
}


// Function borrows value
fn borrow_value(text: &String) {

    println!("Borrowed Value: {}", text);
}


// Mutable borrowing
fn change_username(name: &mut String) {

    name.push_str(" Ekeh");
}