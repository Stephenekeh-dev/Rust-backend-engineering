fn main() {

    println!("========== BASIC LIFETIME EXAMPLE ==========");

    let string1 = String::from("Rust");

    let string2 = String::from("Programming");

    let result = longest_string(&string1, &string2);

    println!("Longest String: {}", result);


    println!("\n========== LIFETIME WITH STRUCT ==========");

    let course = String::from("Rust Backend Engineering");

    let student = Student {

        name: &course,
    };

    println!("Student Course: {}", student.name);


    println!("\n========== LIFETIME SCOPE ==========");

    let first_name = String::from("Steven");

    {
        let second_name = String::from("Ekeh");

        let longest = longest_string(&first_name, &second_name);

        println!("Longest Name: {}", longest);
    }
}


// Function with lifetime annotation
fn longest_string<'a>(
    first: &'a str,
    second: &'a str,
) -> &'a str {

    if first.len() > second.len() {

        first

    } else {

        second
    }
}


// Struct with lifetime annotation
struct Student<'a> {

    name: &'a str,
}