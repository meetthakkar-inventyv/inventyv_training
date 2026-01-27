# Rust Loop Control Flow Demonstration

## Description
This project demonstrates how to use different loop constructs in Rust (`loop`, `while`, and `for`) along with control flow keywords like `break` and `continue`.  
The goal is to print **even numbers from 1 to 10 (exclusive)** while **skipping odd numbers** and **stopping execution when the value reaches 8**.

## Problem Statement
Print even numbers starting from `1` up to `10` (exclusive), but:
- Skip all odd numbers
- Stop the loop when the number becomes `8`

Expected output:

- 2
- 4
- 6


## Concepts Covered
- **loop**: Infinite loop with explicit `break` and `continue`
- **while loop**: Condition-based loop execution
- **for loop**: Iteration over a numeric range
- **break**: Stops loop execution
- **continue**: Skips current iteration

## Code Walkthrough

### 1. Using `loop`
- Starts from `1` and increments manually.
- Uses `continue` to skip odd numbers.
- Uses `break` to exit the loop when the value reaches `8`.
- Demonstrates full control over loop execution.

### 2. Using `while`
- Runs while the counter is less than `10`.
- Increments the counter inside the loop.
- Skips odd numbers using `continue`.
- Stops execution using `break` when the value is `8`.

### 3. Using `for`
- Iterates over a range from `1` to `10` (exclusive).
- Automatically handles incrementing.
- Skips odd values using `continue`.
- Breaks the loop when the value is `8`.


## Output
All three loops produce the same output:

- 2
- 4
- 6