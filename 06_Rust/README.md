# Rust Training Modules

This directory contains hands-on Rust programming assignments created to understand core Rust concepts such as control flow, structs, ownership, serialization, concurrency, and modular design using Cargo projects.

Each subfolder is a **standalone Rust project** with its own `Cargo.toml`, source code, and documentation.

---

## Folder Structure

```text
06_Rust/
│
├── 1_control_flow/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── README.md
│
├── 2_structs_and_methods/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── README.md
│
├── 3_serde_json/
│   ├── 3_1_serde_serialization/
│   │   ├── src/
│   │   │   └── main.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   └── 3_2_serde_deserialization/
│       ├── src/
│       │   └── main.rs
│       ├── Cargo.toml
│       └── README.md
│
├── 4_ownership_and_borrowing/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── README.md
│
├── 5_Request_Tracking_Assignment/
│   ├── src/
│   │   ├── main.rs
│   │   ├── mutex.rs
│   │   └── rwlock.rs
│   ├── Cargo.toml
│   └── README.md
│
└── 6_Modules_Assignment/
    ├── src/
    │   ├── main.rs
    │   └── modules/
    │       ├── mod.rs
    │       ├── control_flow.rs
    │       ├── structs_and_methods.rs
    │       ├── serde_serialization.rs
    │       ├── serde_deserialization.rs
    │       ├── req_track.rs
    │       ├── req_track_mutex.rs
    │       └── req_track_rwlock.rs
    ├── Cargo.toml
    └── README.md
```
---

## Module Overview

### 1_control_flow
Introduces Rust control flow constructs such as loops, conditionals, and execution logic.  
Focuses on understanding program flow and basic Rust syntax through simple examples.

---

### 2_structs_and_methods
Covers defining structs and implementing methods using `impl` blocks.  
Demonstrates setters, getters, ownership of fields, cloning, and structured data modeling.

---

### 3_serde_json
Explores JSON handling in Rust using the `serde` ecosystem.  
Includes separate projects for serialization and deserialization to understand data exchange between Rust and JSON.

---

### 4_ownership_and_borrowing
Provides hands-on practice with Rust’s ownership model and borrowing rules.  
Demonstrates mutable and immutable references, memory safety, and lifetime behavior.

---

### 5_Request_Tracking_Assignment
Implements a request tracking system using concurrency primitives.  
Demonstrates thread-safe shared state management using `Mutex`, `RwLock`, and atomic counters.

---

### 6_Modules_Assignment
Focuses on Rust’s module system and project organization.  
Demonstrates splitting logic into multiple modules and integrating them using `mod.rs` and a single `main.rs`.

---
