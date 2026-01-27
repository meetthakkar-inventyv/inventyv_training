use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
struct Instructor {
    name: String,
    experience_years: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
struct Course {
    title: String,
    instructor: Instructor,
    category: String,
    duration_hours: u16,
    enrolled_students: u32,
    is_certified: bool,
    platform: String,
}

fn main() {
    let course1 = Course {
        title: "Rust for Backend".to_string(),
        instructor: Instructor {
            name: "Meet Thakkar".to_string(),
            experience_years: 6,
        },
        category: "Programming".to_string(),
        duration_hours: 40,
        enrolled_students: 1200,
        is_certified: true,
        platform: "Udemy".to_string(),
    };

    println!();

    // Serialize struct to JSON string
    let json_string_ser = serde_json::to_string(&course1).unwrap();
    println!("json_string_ser : {json_string_ser}");

    println!();

    // Deserialize from JSON string
    let json_from_str_deser: Course = serde_json::from_str(&json_string_ser).unwrap();
    println!("json_from_str_deser : {:#?}", json_from_str_deser);

    println!();

    // Deserialize from raw JSON string
    let raw_string = r#"
    {
        "title": "Advanced Web Development",
        "instructor": {
            "name": "Technical Trainer",
            "experience_years": 10
        },
        "category": "Web",
        "duration_hours": 55,
        "enrolled_students": 2500,
        "is_certified": false,
        "platform": "Coursera"
    }
    "#;

    let json_from_raw_string: Course = serde_json::from_str(raw_string).unwrap();
    println!("json_from_raw_string : {:#?}", json_from_raw_string);

    println!();
}
