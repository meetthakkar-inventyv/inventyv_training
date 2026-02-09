# Inventory Management using Lifetimes & Closures

This project demonstrates a **generic inventory system in Rust** that stores **borrowed data using lifetimes** and displays items using **closures**.

---

## Features
- Generic `Inventory<'a, T>` backed by `HashMap<String, &'a T>`
- Lifetime-based borrowing instead of ownership
- Custom `DisplayItem` trait for item display
- Closures for displaying inventory contents
- Custom error handling with `Result`

---

## Core Concepts Used
- Lifetimes (`'a`)
- Borrowing & References
- Generics with Trait Bounds
- Traits
- Closures
- `HashMap`
- Custom Error Handling

---

## Key Components

### `DisplayItem` Trait
```rust
trait DisplayItem {
    fn display(&self) -> String;
}
```
---

### `Inventory<'a t>` Struct

```rust
struct Inventory<'a, T>
where
    T: DisplayItem + Clone + 'a,
{
    items: HashMap<String, &'a T>,
}
```

---

---

## Supported Operations
- `add_item` - Add borrowed items with ID Validatio
- `find_by_id` - Retrieve an item by ID
- `display_all` - Display all items using closure
---

## Error Handling

Handled Via `InventoryError`:
- `DuplicateId`
- `InvalidId`
- `ItemNotFound`

---


## Example Usage

The inventory works with multiple item types such as:
- `String`
- Custom structs like `Product`
