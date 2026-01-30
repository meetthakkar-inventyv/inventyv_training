# Rust Request Tracking (Mutex & RwLock)

---

## Problem Statement

1. Define an enum called `Request` with the following variants:
   - `Get { endpoint: String }`
   - `Post { endpoint: String, payload_size: u32 }`
   - `Delete(u32)` → represents deleting a resource by ID

2. Use a static variable to keep track of the total number of requests processed.

3. Write a function `handle_request(req: Request) -> String` that:
   - Uses pattern matching (`match`)
   - Increments the static request counter
   - Returns a meaningful message for each request type

4. In `main()`:
   - Create at least one request of each type
   - Process them using `handle_request`
   - Print the response and total request count

---

## Project Overview

This project demonstrates how to safely manage shared state in Rust using `AtomicUsize` along with two synchronization primitives: `Mutex` and `RwLock`.

### Implementation Details

- `main.rs` acts as the entry point and invokes both implementations.
- `mutex.rs` tracks GET, POST, and DELETE requests using `Mutex` for exclusive access.
- `rwlock.rs` tracks the same requests using `RwLock`, allowing multiple readers and exclusive writers.
- `AtomicUsize` is used to maintain a lock-free total request count.

---


