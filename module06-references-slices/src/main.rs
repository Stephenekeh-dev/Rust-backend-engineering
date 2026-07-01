fn main() {

    println!("========== REFERENCES ==========");

    let name = String::from("Steven");

    // Creating a reference
    let reference = &name;

    println!("Original Value: {}", name);
    println!("Reference Value: {}", reference);


    println!("\n========== IMMUTABLE REFERENCE ==========");

    let course = String::from("Rust Programming");

    print_course(&course);

    // course is still valid
    println!("Main Function: {}", course);


    println!("\n========== MUTABLE REFERENCE ==========");

    let mut username = String::from("Steven");

    update_username(&mut username);

    println!("Updated Username: {}", username);


    println!("\n========== STRING SLICES ==========");

    let message = String::from("Rust Backend Engineering");

    let first_word = &message[0..4];

    let backend_word = &message[5..12];

    println!("First Word: {}", first_word);
    println!("Second Word: {}", backend_word);


    println!("\n========== ARRAY SLICES ==========");

    let numbers = [10, 20, 30, 40, 50];

    let slice = &numbers[1..4];

    println!("Array Slice: {:?}", slice);
}


// Immutable reference
fn print_course(course: &String) {

    println!("Borrowed Course: {}", course);
}


// Mutable reference
fn update_username(name: &mut String) {

    name.push_str(" Ekeh");
}