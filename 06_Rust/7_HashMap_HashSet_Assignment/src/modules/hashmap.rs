use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Student{
    id: u32,
    name: String,
}

pub fn run(){
  let mut students : HashMap<u32,Student> = HashMap::new();

    students.insert(1, Student { id: 1, name: "Meet".to_string(), });
    students.insert(2, Student { id: 2, name: "jaimin".to_string(), });
    students.insert(3, Student { id: 3, name: "prem".to_string(), });

    println!("Original HashMap: {:#?}", students);

    //clone
    let students_clone = students.clone();
    println!("---------Using Clone--------\n{:#?}", students_clone);

    //try_reserve
    match students.try_reserve(10) {
        Ok(()) => println!("Successfully reserved capacity"),
        Err(e) => println!("Error: {}", e),
    }

    //take
    // let taken_student : Vec<(u32, Student)> = students.into_iter().take(1).collect();
    //     println!("---------Using Take--------\n{:#?}", taken_student);

    let taken_students = std::mem::take(&mut students);
    println!("---------Using Take--------\n{:#?}", taken_students);
    println!("original students {:#?}", students); //this is empty {} becasue data is moved into taken_students using Default trait

    //extend
    students.extend(taken_students);
    println!("---------Using Extend--------\n{:#?}", students);

    //remove
    let first_student  = students.remove(&1);

    match &first_student {
        Some(student) => println!("---------Using Remove--------\n{:#?}", student),
        None => println!("No student found"),
    }
    println!("---------After Remove--------\n{:#?}", students);

    //retain
    students.insert(1, Student { id: 1, name: "Meet".to_string(), });
    students.retain(|_key , student| { //this is closure (Anonymus function) like Arrow function we will see letter about this 
        student.id == 1   
    });
    println!("---------Using Retain--------\n{:#?}", students);


}