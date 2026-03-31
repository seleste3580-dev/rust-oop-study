/* combining structs enums unions dynamic mem allocation 
using box for heap allocation and dynamic dispatch */

//Students struct
struct Student {
    name: String,
    age: u32,
    grade: f32,
}
//Teacher struct
struct Teacher {
    name: String,  
    subject: String,
    experience: u32,
}
//Person trait
trait Person {
    fn greet(&self);    
    fn info(&self) -> String;
}
//enums to differentiate roles
/* 
enum Role {
    Student,
    Teacher,
}
//union for extra info (unsafe in Rust, but for demonstration)
union ExtraInfo {
    grade: f32,
    experience: u32,
}
//implimenting method for Student

impl Student {
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy Birthday, {}! You are now {} years old.", self.name, self.age);
    }
}
    */
//implimenting Person trait for Student
impl Person for Student {
    fn greet(&self) {
        println!("Hello, my name is {}!", self.name);
    }
    fn info(&self) -> String {
        format!("{} is {} years old and has a grade of {}.", self.name, self.age, self.grade)
    }
}
//implimenting Person trait for Teacher
impl Person for Teacher {
    fn greet(&self) {
        println!("Hello, my name is {} and I teach {}!", self.name, self.subject);
    }
    fn info(&self) -> String {
        format!("{} teaches {} and has {} years of experience.", self.name, self.subject, self.experience)
    }
}
    
fn main() {
    //dynamic mem allocation on the heap using Box
    let student1 = Box::new(Student {
        name: String::from("Alice"),
        age: 20,
        grade: 85.5,
    });
    let student2 = Box::new(Student {
        name: String::from("Bob"),
        age: 22,
        grade: 55.0,
    });
    let teacher1 = Box::new(Teacher {
        name: String::from("Mr. Smith"),
        subject: String::from("Math"),
        experience: 10,
    });
    let teacher2 = Box::new(Teacher {
        name: String::from("Ms. Johnson"),
        subject: String::from("History"),
        experience: 5,
    });
    //collection o different types using dynamics dispatch with trait objects
    let people: Vec<Box<dyn Person>> = vec![
        student1, 
        student2,
        teacher1, 
        teacher2
         ];
    println!("----Greetings and info:----");
    for person in &people {
        person.greet(); //dynamic dispatch to call the correct greet method based on the actual type of person
        println!("{}", person.info());
    }
    /* 
    //example modifying a student to have a birthday (requires downcasting, which is unsafe and not recommended in real code)
    println!("\n---Celebrating birthdays:---");
    for person in people {
        //downcasting to Student to call have_birthday (unsafe and for demonstration only)
        if let Some(student) = person.as_any_mut().downcast_mut::<Student>() {
            let mut student = student.clone(); //clone to modify
            student.have_birthday();
        }
    }
    
    for person in &people {
        println!("{}", person.info());
    }
    //Extension trait to allow downcasting from dynamic trait objects (not recommended in production code)
   println!("\n---Updated info:---");
     use std::any::Any;
    trait AsAny {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }
    impl<T: 'static> AsAny for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
    //extend dynamic trait objects to support downcasting (unsafe and for demonstration only)
    impl dyn Person {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
    */

}