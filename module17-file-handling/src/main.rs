use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {

    println!("========== CREATING FILE ==========");

    let mut file = File::create("students.txt")
        .expect("Failed to create file");

    println!("File created successfully");


    println!("\n========== WRITING TO FILE ==========");

    file.write_all(
        b"Steven - Rust Backend Engineering\n"
    )
    .expect("Failed to write to file");

    file.write_all(
        b"John - Systems Programming\n"
    )
    .expect("Failed to write to file");

    println!("Data written successfully");


    println!("\n========== READING FILE ==========");

    let mut file = File::open("students.txt")
        .expect("Failed to open file");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Failed to read file");

    println!("File Content:\n{}", content);


    println!("\n========== APPENDING TO FILE ==========");

    let mut append_file = OpenOptions::new()
        .append(true)
        .open("students.txt")
        .expect("Failed to open file for appending");

    append_file.write_all(
        b"Mary - Cybersecurity\n"
    )
    .expect("Failed to append data");

    println!("New data appended successfully");


    println!("\n========== UPDATED FILE CONTENT ==========");

    let mut updated_file = File::open("students.txt")
        .expect("Failed to reopen file");

    let mut updated_content = String::new();

    updated_file
        .read_to_string(&mut updated_content)
        .expect("Failed to read updated file");

    println!("{}", updated_content);
}