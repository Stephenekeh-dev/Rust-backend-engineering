fn main() {

    println!("========== GENERIC FUNCTION ==========");

    let int_result = largest_number(10, 20);

    let float_result = largest_number(15.5, 9.3);

    println!("Largest Integer: {}", int_result);

    println!("Largest Float: {}", float_result);


    println!("\n========== GENERIC STRUCT ==========");

    let integer_point = Point {

        x: 10,

        y: 20,
    };

    let float_point = Point {

        x: 5.5,

        y: 8.2,
    };

    println!(
        "Integer Point => x: {}, y: {}",
        integer_point.x,
        integer_point.y
    );

    println!(
        "Float Point => x: {}, y: {}",
        float_point.x,
        float_point.y
    );


    println!("\n========== GENERIC METHOD ==========");

    println!(
        "X Coordinate: {}",
        integer_point.get_x()
    );


    println!("\n========== MULTIPLE GENERIC TYPES ==========");

    let mixed_data = Data {

        name: String::from("Steven"),

        age: 25,
    };

    println!(
        "Name: {}, Age: {}",
        mixed_data.name,
        mixed_data.age
    );
}


// Generic Function
fn largest_number<T: PartialOrd>(a: T, b: T) -> T {

    if a > b {

        a

    } else {

        b
    }
}


// Generic Struct
struct Point<T> {

    x: T,

    y: T,
}


// Generic Implementation Block
impl<T> Point<T> {

    fn get_x(&self) -> &T {

        &self.x
    }
}


// Multiple Generic Types
struct Data<T, U> {

    name: T,

    age: U,
}