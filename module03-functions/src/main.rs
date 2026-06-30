fn main() {
  custom_function();
  with_parameter(15); 
  let area:i32 = area(3, 4);
  println!("the value returned by the method area() is  {}", area);
}

// Custom function 
fn custom_function(){
    println!(" This is a user created function");
}
// Custom function with parameter
fn with_parameter(x:i32) {
    println!("This is the parameter {}", x);
} 
// Function with parameter and reutrn type
fn area(x:i32, y:i32) -> i32 {
    x * y
}

