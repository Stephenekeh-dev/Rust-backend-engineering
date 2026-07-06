fn main() {

    println!("========== BASIC MATCH ==========");

    let number = 3;

    match number {

        1 => println!("One"),

        2 => println!("Two"),

        3 => println!("Three"),

        _ => println!("Unknown Number"),
    }


    println!("\n========== MATCH WITH ENUM ==========");

    let user_role = UserRole::Admin;

    match user_role {

        UserRole::Admin =>{
            println!("Full system access");
        }

        UserRole::Moderator => {
            println!("Limited administrative access");
        }

        UserRole::User => {
            println!("Regular user access");
        }
    }


    println!("\n========== MATCH WITH ENUM DATA ==========");

    let payment = Payment::Card(String::from("Visa"), 1234);

    match payment {

        Payment::Cash => {
            println!("Cash Payment");
        }

        Payment::Transfer(bank) => {
            println!("Bank Transfer from {}", bank);
        }

        Payment::Card(card_type, digits) => {
            println!(
                "Card Payment using {} ending with {}",
                card_type,
                digits
            );
        }
    }


    println!("\n========== IF LET ==========");

    let account_status = Status::Active;

    if let Status::Active = account_status {

        println!("Account is active");
    }


    println!("\n========== WHILE LET ==========");

    let mut stack = vec![1, 2, 3, 4];

    while let Some(top) = stack.pop() {

        println!("Popped Value: {}", top);
    }
}


// Enum for user roles
enum UserRole {

    Admin,

    Moderator,

    User,
}


// Enum with associated data
enum Payment {

    Cash,

    Transfer(String),

    Card(String, u32),
}


// Enum for if let example
enum Status {

    Active,

    Inactive,
}