fn main() {

    println!("========== IMPLEMENTING TRAITS ==========");

    let student = Student {

        name: String::from("Steven"),

        course: String::from("Rust Backend Engineering"),
    };

    let teacher = Teacher {

        name: String::from("John"),

        subject: String::from("Systems Programming"),
    };

    student.introduce();

    teacher.introduce();


    println!("\n========== TRAIT WITH RETURN VALUE ==========");

    println!("{}", student.get_role());

    println!("{}", teacher.get_role());


    println!("\n========== TRAIT BOUNDS ==========");

    display_information(&student);

    display_information(&teacher);
}


// Trait Definition
trait Person {

    fn introduce(&self);

    fn get_role(&self) -> String;
}


// Student Struct
struct Student {

    name: String,

    course: String,
}


// Teacher Struct
struct Teacher {

    name: String,

    subject: String,
}


// Implement Trait for Student
impl Person for Student {

    fn introduce(&self) {

        println!(
            "Student Name: {}, Course: {}",
            self.name,
            self.course
        );
    }

    fn get_role(&self) -> String {

        String::from("Role: Student")
    }
}


// Implement Trait for Teacher
impl Person for Teacher {

    fn introduce(&self) {

        println!(
            "Teacher Name: {}, Subject: {}",
            self.name,
            self.subject
        );
    }

    fn get_role(&self) -> String {

        String::from("Role: Teacher")
    }
}


// Function Using Trait Bound
fn display_information<T: Person>(person: &T) {

    person.introduce();

    println!("{}", person.get_role());
}