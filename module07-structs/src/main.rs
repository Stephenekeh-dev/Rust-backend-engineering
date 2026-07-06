fn main() {

    println!("========== BASIC STRUCT ==========");

    let user1 = User {

        username: String::from("steven"),

        email: String::from("steven@gmail.com"),

        age: 25,

        active: true,
    };

    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Age: {}", user1.age);
    println!("Active: {}", user1.active);


    println!("\n========== MUTABLE STRUCT ==========");

    let mut employee = Employee {

        name: String::from("John"),

        salary: 5000.0,
    };

    println!("Old Salary: {}", employee.salary);

    employee.salary = 7000.0;

    println!("Updated Salary: {}", employee.salary);


    println!("\n========== STRUCT UPDATE SYNTAX ==========");

    let user2 = User {

        username: String::from("mary"),

        email: String::from("mary@gmail.com"),
        age:24,
        ..user1
    };

    println!("New Username: {}", user2.username);
    println!("New Email: {}", user2.email);
    println!("the age is {}", user2.age);
    println!("The age is {}", user1.active);


    println!("\n========== TUPLE STRUCT ==========");

    let coordinates = Point(10, 20);

    println!("X Coordinate: {}", coordinates.0);
    println!("Y Coordinate: {}", coordinates.1);


    println!("\n========== UNIT-LIKE STRUCT ==========");

    let _person = Person;

    println!("Unit-like struct created successfully");
}


// Basic Struct
struct User {

    username: String,

    email: String,

    age: u32,

    active: bool,
}


// Mutable Struct Example
struct Employee {

    name: String,

    salary: f64,
}


// Tuple Struct
struct Point(i32, i32);


// Unit-like Struct
struct Person;