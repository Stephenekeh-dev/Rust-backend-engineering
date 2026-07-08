mod math;

mod greetings;

fn main() {

    println!("========== MODULES ==========");

    let sum = math::add_numbers(10, 20);

    let product = math::multiply_numbers(5, 4);

    println!("Sum: {}", sum);

    println!("Product: {}", product);


    println!("\n========== GREETINGS MODULE ==========");

    greetings::welcome_message("Steven");

    greetings::course_message();
}