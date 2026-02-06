# Inventory Management

This project demonstrates a **generic inventory management system in Rust** using traits, generics, and custom error handling.

## Features
- Generic `Inventory<T>` backed by `HashMap<String, T>`
- Custom `DisplayItem` trait for item-specific display logic
- Strong type safety using trait bounds (`T: DisplayItem + Clone`)
- Custom `InventoryError` enum for robust error handling

## Core Concepts Used
- Rust Generics
- Trait Bounds
- Custom Traits
- `Result`-based Error Handling
- `HashMap` for fast lookups

## Key Components

### `DisplayItem` Trait
```rust
trait DisplayItem {
    fn display(&self) -> String;
}
```
---
### `Inventory<T>` Struct
```rust
struct Inventory<T>
where
    T: DisplayItem + Clone,
{
    items: HashMap<String, T>,
}
```
---

## Supported Operations
- `add_item` - Add an item with duplicate and ID validation
- `find_by_id` - Retrieve an item by ID
- `display_all` - Display all items using trait-based formatting

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

