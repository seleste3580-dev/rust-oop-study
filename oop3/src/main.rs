struct Student {
    name: String,
    age: u32,
    grade: f32,
}
impl Student {
    //method to display student information
    fn display(&self) {
        println!("Name: {}, Age: {}, Grade: {}", self.name, self.age, self.grade);
    }
    //method to increment age
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy Birthday, {}! You are now {} years old.", self.name, self.age);
    }
    fn has_passed(&self) -> bool {
        self.grade >= 60.0
    }
}
fn main() {
   let mut students = vec! [
        Student {
            name: String::from("Alice"),
            age: 20,
            grade: 85.5,
        },
        Student {
            name: String::from("Bob"),
            age: 22,
            grade: 55.0,
        },
        Student {
            name: String::from("Charlie"),
            age: 19,
            grade: 72.0,
        },
   ];
   
    for student in &students {
        student.display();
    }

    for student in &mut students {
        student.have_birthday();
    }

    for student in &students {
        println!("Has {} passed? {}", student.name, student.has_passed());
    }
}