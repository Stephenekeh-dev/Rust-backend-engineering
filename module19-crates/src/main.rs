use rand::Rng;

use module19_crates::{
    add_numbers,
    greet_user,
};

fn main() {

    println!("========== LIBRARY CRATE ==========");

    greet_user("Steven");

    let result = add_numbers(10, 20);

    println!("Addition Result: {}", result);


    println!("\n========== EXTERNAL CRATE ==========");

    let mut rng = rand::thread_rng();

    let random_number = rng.gen_range(1..101);

    println!(
        "Random Number: {}",
        random_number
    );
}