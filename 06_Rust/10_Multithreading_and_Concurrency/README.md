# Multithreaded Shared State Application


## Problem Statement : 

Build a Rust application that runs 6 concurrent threads operating on shared in-memory data.  
The system maintains a global atomic counter for unique record IDs, stores records in a shared collection, periodically creates new records, prints state, removes expired even/odd records (older than 20 seconds), and counts even/odd records concurrently.

---

## Overview

This project demonstrates a multithreaded Rust application using:

- `Arc` for shared ownership
- `Mutex` for safe mutable access
- `AtomicI32` for a global counter
- `chrono` for timestamp handling

Six concurrent threads operate on shared in-memory data.

---

## Data Structure

```rust
struct MultiThread {
    id: i32,
    record_added_time: String,
    thread_id: String,
}
```
## Records are stored inside

```rust
Arc<Mutex<Vec<MultiThread>>>
```
---

## Thread Responsibilities

### Record Creator
- Runs every 10 seconds  
- Generates a unique ID  
- Adds a timestamped record  

### State Printer
- Runs every 5 seconds  
- Prints total records and data  

### Even Cleaner
- Runs every 5 seconds  
- Removes even ID records older than 20 seconds  

### Odd Cleaner
- Runs every 5 seconds  
- Removes odd ID records older than 20 seconds  

### Even Counter
- Runs every 7 seconds  
- Counts even ID records  

### Odd Counter
- Runs every 7 seconds  
- Counts odd ID records  

---

## Key Concepts Demonstrated

- Thread spawning with `std::thread`  
- Shared state using `Arc<Mutex<T>>`  
- Atomic operations using `AtomicI32`  
- Time comparison using `chrono`  
- Concurrent read/write synchronization  
