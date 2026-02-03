# Ownership and Borrowing

## Overview
This project demonstrates **Rust ownership, borrowing, and mutable references** using an `Order` management example.

## Objective
- Modify struct data using **mutable references (`&mut self`)**
- Observe how ownership is preserved while mutating data
- After every `.set()` call, print the updated struct **twice**

## Key Concepts Used
- Ownership and borrowing
- Mutable references
- Nested structs (`Address`, `Payment`)
- Setters using `&mut self`
- Debug printing with `self` and `&self`

## Behavior
- Setter methods update data using mutable borrowing
- After each update, the full struct is printed:
  - Once using `self`
  - Once using `&self`
- This proves both references point to the same owned data

## Conclusion
The project shows how Rust allows **safe data mutation** using mutable references while strictly enforcing ownership rules.
