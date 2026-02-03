use std::collections::HashSet;

#[derive(Debug, Clone,Eq,PartialEq,Hash)]
#[allow(dead_code)]
pub struct Student {
    name: String,
    id: u32
}
pub fn run(){
   let mut students : HashSet<Student> = HashSet::new();

    students.insert(Student {name : "Meet".to_string(), id: 1});
    students.insert(Student {name : "Jaimin".to_string(), id: 2});
    students.insert(Student {name : "Prem".to_string(), id: 3});

    println!("---------Original HashSet--------\n{:#?}", students);

    //clone
    let students_clone = students.clone();
    println!("---------Using Clone--------\n{:#?}", students_clone);

 
    //try_reserve
    match students.try_reserve(10) {
        Ok(()) => println!("Successfully reserved capacity"),
        Err(e) => println!("Error: {}", e),
    }

    //take
    let taken_name = students.take(&Student {name : "Meet".to_string(), id: 1});
    match taken_name {
        Some(student) => println!("---------Using Take--------\n{:#?}", student),
        None => println!("No student found"),
    }

    let taken_students = std::mem::take(&mut students);
    println!("---------Using Take--------\n taken Students: {:#?}", taken_students);
    println!("original students {:#?}", students); //this is empty {} becasue data is moved into taken_names

    //extend
    students.extend(taken_students);
    println!("---------Using Extend--------\n students: {:#?}", students);

    students.extend([Student {name : "Prem".to_string(), id: 3},Student {name : "Jashmin".to_string(), id: 4}]); //duplicate value will be ignored
    println!("---------Using Extend But Adding Values--------\n names: {:#?}", students);

    
    //retain
    students.retain(|student| {
        student.name.starts_with('J')
    });
    println!("---------Using Retain--------\n names: {:#?}", students);
}