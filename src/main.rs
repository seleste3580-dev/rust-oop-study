use std::io;

struct Student {
    name: String,
    age: u32,
    grade: f32,
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    let n: usize = read_input("Enter the number of students:")
        .parse()
        .expect("Please enter a valid number");

    let mut students: Vec<Student> = Vec::new();

    for i in 1..=n {
        println!("\nEnter details for student {}:", i);

        let name = read_input("Name:");
        let age: u32 = read_input("Age:")
            .parse()
            .expect("Please enter a valid age");
        let grade: f32 = read_input("Grade:")
            .parse()
            .expect("Please enter a valid grade");

        students.push(Student { name, age, grade });
    }

    println!("\nStudent records:");
    for (index, student) in students.iter().enumerate() {
        println!(
            "Student {}: Name: {}, Age: {}, Grade: {}",
            index + 1,
            student.name,
            student.age,
            student.grade
        );
    }
}