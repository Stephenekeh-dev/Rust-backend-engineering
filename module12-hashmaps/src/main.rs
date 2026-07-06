use std::collections::HashMap;

fn main() {

    println!("========== CREATING HASHMAP ==========");

    let mut student_scores = HashMap::new();

    student_scores.insert("Steven", 90);
    student_scores.insert("John", 85);
    student_scores.insert("Mary", 95);

    println!("Student Scores: {:?}", student_scores);


    println!("\n========== ACCESSING VALUES ==========");

    match student_scores.get("Steven") {

        Some(score) => {
            println!("Steven's Score: {}", score);
        }

        None => {
            println!("Student not found");
        }
    }


    println!("\n========== ITERATING THROUGH HASHMAP ==========");

    for (name, score) in &student_scores {

        println!("{} => {}", name, score);
    }


    println!("\n========== UPDATING VALUES ==========");

    student_scores.insert("John", 100);

    println!("Updated Scores: {:?}", student_scores);


    println!("\n========== CHECKING FOR EXISTING KEY ==========");

    student_scores.entry("Daniel").or_insert(70);

    student_scores.entry("Steven").or_insert(50);

    println!("After Entry Update: {:?}", student_scores);


    println!("\n========== REMOVING VALUES ==========");

    student_scores.remove("Mary");

    println!("After Removal: {:?}", student_scores);


    println!("\n========== COUNTING WORD OCCURRENCES ==========");

    let text = "rust java rust python rust java";

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {

        let count = word_count.entry(word).or_insert(0);

        *count += 1;
    }

    println!("Word Count: {:?}", word_count);
}