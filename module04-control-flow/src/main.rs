fn main() {

    println!("========== IF STATEMENT ==========");

    let age = 20;

    if age >= 18 {
        println!("You are an adult");
    }

    println!("\n========== IF ELSE ==========");

    let temperature = 15;

    if temperature > 25 {
        println!("The weather is hot");
    } else {
        println!("The weather is cold");
    }

    println!("\n========== ELSE IF ==========");

    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

    println!("\n========== LOOP ==========");

    let mut count = 1;

    loop {

        println!("Loop Count: {}", count);

        count += 1;

        if count > 3 {
            break;
        }
    }

    println!("\n========== WHILE LOOP ==========");

    let mut number = 1;

    while number <= 5 {

        println!("Number: {}", number);

        number += 1;
    }

    println!("\n========== FOR LOOP ==========");

    for i in 1..=5 {

        println!("Value of i: {}", i);
    }

    println!("\n========== LOOPING THROUGH ARRAY ==========");

    let fruits = ["Apple", "Orange", "Mango", "Banana"];

    for fruit in fruits {

        println!("Fruit: {}", fruit);
    }

    println!("\n========== MATCH STATEMENT ==========");

    let day = 3;

    match day {

        1 => println!("Monday"),

        2 => println!("Tuesday"),

        3 => println!("Wednesday"),

        4 => println!("Thursday"),

        5 => println!("Friday"),

        _ => println!("Weekend"),
    }
}