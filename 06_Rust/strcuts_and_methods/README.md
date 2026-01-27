# Rust Structs and Order Management System

## Description
This project demonstrates the use of **Structs** in Rust to model a real-world **Order Management System**. It covers defining structures, nesting structs, implementing methods (getters and setters), cloning data, and using associated functions (static methods) to generate formatted output.

The example is designed to closely resemble real-world scenarios such as e-commerce or delivery platforms.

## Concepts Covered
- **Struct Definition**: Defining custom data types (`Order`, `Address`, and `Payment`).
- **Nested Structs**: Using multiple structs (`Address` and `Payment`) as fields inside another struct (`Order`).
- **Impl Block**: Implementing functionality and behavior for structs.
- **Methods**:
  - **Getters**: Accessing data using `&self`.
  - **Setters**: Modifying data using `&mut self`.
- **Associated Functions**: Functions defined inside an `impl` block that do not take `self` as a parameter (similar to static methods).
- **Clone Trait**: Duplicating struct instances safely.
- **Format Macro**: Using `format!` to construct structured, readable output strings.

## Code Walkthrough

### 1. Struct Definitions
- **Address**
  - Stores delivery location details such as `city`, `state`, and `country`.

- **Payment**
  - Stores payment-related information including payment `method`, `transaction_id`, and `amount`.

- **Order**
  - Represents a customer order and includes:
    - Order ID
    - Customer name
    - Delivery address
    - Payment details
    - Total number of items
    - Delivery status
    - Platform name

All structs derive `Debug` for formatted output and `Clone` to allow copying of instances.

### 2. Method Implementations
The `impl Order` block defines multiple methods to interact with order data.

#### Setter Methods
- Methods such as `set_order_id`, `set_customer_name`, `set_city`, `set_payment_method`, and others.
- Use `&mut self` to update values.
- Demonstrate modifying both direct fields and nested struct fields.

#### Getter Methods
- Methods such as `get_order_id`, `get_customer_name`, `get_city`, `get_payment_method`, etc.
- Use `&self` to safely access data.
- `get_delivery_status` returns a descriptive string (`"Order Delivered"` or `"Order Pending"`) instead of a raw boolean.

#### Full Information Method
- **`get_full_order`**
  - Returns a multi-line formatted string containing all order details in a readable format.

### 3. Associated Function
- **`get_full_order_wo_self`**
  - Does not require an `Order` instance.
  - Accepts all required fields as parameters.
  - Returns a formatted order summary string.
  - Demonstrates how associated functions can be used without object creation.

### 4. Main Execution
- Creates an initial `Order` instance with sample data.
- Prints full order details using `get_full_order`.
- Clones the order into a new variable.
- Updates delivery status and payment amount using setter methods.
- Prints updated order details.
- Calls `Order::get_full_order_wo_self` directly to generate order information without using `self`.


This project provides a strong foundation for understanding struct-based data modeling and method implementation in Rust.
