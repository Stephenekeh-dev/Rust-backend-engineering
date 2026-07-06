fn main() {

    println!("========== OPTION ENUM ==========");

    let user1 = find_user(1);

    let user2 = find_user(5);

    match user1 {

        Some(name) => {
            println!("User Found: {}", name);
        }

        None => {
            println!("User does not exist");
        }
    }

    match user2 {

        Some(name) => {
            println!("User Found: {}", name);
        }

        None => {
            println!("User does not exist");
        }
    }


    println!("\n========== IF LET WITH OPTION ==========");

    let course = get_course(true);

    if let Some(course_name) = course {

        println!("Course Available: {}", course_name);
    }


    println!("\n========== RESULT ENUM ==========");

    let division1 = divide(10.0, 2.0);

    let division2 = divide(10.0, 0.0);

    match division1 {

        Ok(value) => {
            println!("Division Result: {}", value);
        }

        Err(error) => {
            println!("Error: {}", error);
        }
    }

    match division2 {

        Ok(value) => {
            println!("Division Result: {}", value);
        }

        Err(error) => {
            println!("Error: {}", error);
        }
    }
}


// Function returning Option
fn find_user(id: u32) -> Option<String> {

    if id == 1 {

        Some(String::from("Steven"))

    } else {

        None
    }
}


// Another Option example
fn get_course(available: bool) -> Option<String> {

    if available {

        Some(String::from("Rust Backend Engineering"))

    } else {

        None
    }
}


// Function returning Result
fn divide(a: f64, b: f64) -> Result<f64, String> {

    if b == 0.0 {

        Err(String::from("Cannot divide by zero"))

    } else {

        Ok(a / b)
    }
}