fn main() {

    println!("========== BASIC ENUM ==========");

    let user_status = AccountStatus::Active;

    display_status(user_status);


    println!("\n========== ENUM WITH DATA ==========");

    let payment1 = PaymentMethod::Cash;

    let payment2 = PaymentMethod::Transfer(String::from("GTBank"));

    let payment3 = PaymentMethod::Card(String::from("Visa"), 1234);

    process_payment(payment1);

    process_payment(payment2);

    process_payment(payment3);


    println!("\n========== ENUM INSIDE STRUCT ==========");

    let order = Order {

        id: 101,

        status: OrderStatus::Shipped,
    };

    println!("Order ID: {}", order.id);

    match order.status {

        OrderStatus::Pending => println!("Order Pending"),

        OrderStatus::Processing => println!("Order Processing"),

        OrderStatus::Shipped => println!("Order Shipped"),

        OrderStatus::Delivered => println!("Order Delivered"),
    }
}


// Basic Enum
enum AccountStatus {

    Active,

    Inactive,

    Suspended,
}


// Function using enum
fn display_status(status: AccountStatus) {

    match status {

        AccountStatus::Active => {
            println!("Account is Active");
        }

        AccountStatus::Inactive => {
            println!("Account is Inactive");
        }

        AccountStatus::Suspended => {
            println!("Account is Suspended");
        }
    }
}


// Enum with associated data
enum PaymentMethod {

    Cash,

    Transfer(String),

    Card(String, u32),
}


// Function handling enum data
fn process_payment(payment: PaymentMethod) {

    match payment {

        PaymentMethod::Cash => {
            println!("Payment made with Cash");
        }

        PaymentMethod::Transfer(bank) => {
            println!("Bank Transfer from {}", bank);
        }

        PaymentMethod::Card(card_type, last_digits) => {
            println!(
                "Card Payment using {} ending with {}",
                card_type,
                last_digits
            );
        }
    }
}


// Enum inside struct
struct Order {

    id: u32,

    status: OrderStatus,
}


// Order status enum
enum OrderStatus {

    Pending,

    Processing,

    Shipped,

    Delivered,
}