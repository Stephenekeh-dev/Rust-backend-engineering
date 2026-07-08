use std::fs::File;
use std::io::{self, Read};

fn main() {

    println!("========== PANIC EXAMPLE ==========");

    // Uncomment to see panic in action
    // panic!("Something went wrong!");


    println!("\n========== RESULT WITH MATCH ==========");

    match divide_numbers(10.0, 2.0) {

        Ok(result) => {
            println!("Division Result: {}", result);
        }

        Err(error) => {
            println!("Error: {}", error);
        }
    }

    match divide_numbers(10.0, 0.0) {

        Ok(result) => {
            println!("Division Result: {}", result);
        }

        Err(error) => {
            println!("Error: {}", error);
        }
    }


    println!("\n========== UNWRAP EXAMPLE ==========");

    let number = Some(50);

    println!("Unwrapped Value: {}", number.unwrap());


    println!("\n========== EXPECT EXAMPLE ==========");

    let username = Some("Steven");

    println!(
        "Username: {}",
        username.expect("Username not found")
    );


    println!("\n========== ERROR PROPAGATION ==========");

    match read_file_content() {

        Ok(content) => {
            println!("File Content:\n{}", content);
        }

        Err(error) => {
            println!("File Error: {}", error);
        }
    }
}


// Function returning Result
fn divide_numbers(
    a: f64,
    b: f64,
) -> Result<f64, String> {

    if b == 0.0 {

        Err(String::from("Cannot divide by zero"))

    } else {

        Ok(a / b)
    }
}


// Function using ? operator
fn read_file_content() -> Result<String, io::Error> {

    let mut file = File::open("sample.txt")?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}