fn main() {

    println!("========== CREATING VECTORS ==========");

    let numbers = vec![10, 20, 30, 40];

    println!("Numbers Vector: {:?}", numbers);


    println!("\n========== EMPTY VECTOR ==========");

    let mut fruits: Vec<&str> = Vec::new();

    fruits.push("Apple");
    fruits.push("Orange");
    fruits.push("Mango");

    println!("Fruits Vector: {:?}", fruits);


    println!("\n========== ACCESSING VECTOR ELEMENTS ==========");

    let scores = vec![90, 80, 70, 60];

    println!("First Score: {}", scores[0]);

    match scores.get(2) {

        Some(value) => {
            println!("Third Score: {}", value);
        }

        None => {
            println!("No value found");
        }
    }


    println!("\n========== ITERATING THROUGH VECTOR ==========");

    let cities = vec!["Lagos", "Abuja", "Port Harcourt"];

    for city in &cities {

        println!("City: {}", city);
    }


    println!("\n========== MUTABLE ITERATION ==========");

    let mut prices = vec![100, 200, 300];

    for price in &mut prices {

        *price += 50;
    }

    println!("Updated Prices: {:?}", prices);


    println!("\n========== VECTOR LENGTH ==========");

    println!("Total Prices: {}", prices.len());


    println!("\n========== REMOVING ELEMENTS ==========");

    let mut languages = vec!["Rust", "Java", "Python"];

    languages.pop();

    println!("Remaining Languages: {:?}", languages);
}