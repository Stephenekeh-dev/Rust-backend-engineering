fn main() {

    // Variable creation and initailization 
    let name = "Steven";
    println!("Name: {}", name);
    

    //Variables are immutable unless specified mutable 
    let mut age = 25;
    println!("Age before update: {}", age);
    
    //the value assigned to age can be changed because it is specified mutable
    age = 26;
    println!("Age after update: {}", age);

    //Demonstrating shadowing 
    let email = "steve.example.com";
    println!("{}", email);
    //using the same variable name shadows the first one
    let email = "ekeh.example.com";
    println!("{}", email);
}
