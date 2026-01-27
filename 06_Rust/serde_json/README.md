# Rust Serde JSON Example

This project demonstrates how to use the `serde` framework with the `serde_json` crate in Rust to serialize and deserialize data structures to and from JSON format.

## Overview

The application defines nested data structures (`Project` and `Technology`) and performs the following operations:
1.  **Serialization**: Converts a Rust struct instance into a JSON string.
2.  **Deserialization**: Parses a JSON string back into a Rust struct.
3.  **Raw String Handling**: Demonstrates parsing a raw string literal directly into a struct.

## Dependencies

The project uses the following dependencies in `Cargo.toml`:
*   `serde`: For the serialization framework (with `derive` feature enabled).
*   `serde_json`: For JSON-specific implementation.

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```


## Usage

To run the project, ensure you have Rust installed and run:

```bash
cargo run
```

## Expected Output

The program will output:
1.  The serialized JSON string derived from a struct.
2.  The deserialized struct printed using the `Debug` formatter (`{:#?}`).
3.  Another deserialized struct derived from a raw JSON string literal.

```text

json_string_ser : {"title":"Rust for Backend","instructor":{"name":"Meet Thakkar","experience_years":6},"category":"Programming","duration_hours":40,"enrolled_students":1200,"is_certified":true,"platform":"Udemy"}

json_from_str_deser : Course {
    title: "Rust for Backend",
    instructor: Instructor {
        name: "Meet Thakkar",
        experience_years: 6,
    },
    category: "Programming",
    duration_hours: 40,
    enrolled_students: 1200,
    is_certified: true,
    platform: "Udemy",
}

json_from_raw_string : Course {
    title: "Advanced Web Development",
    instructor: Instructor {
        name: "Technical Trainer",
        experience_years: 10,
    },
    category: "Web",
    duration_hours: 55,
    enrolled_students: 2500,
    is_certified: false,
    platform: "Coursera",
}

```