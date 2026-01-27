# Serde Deserialization

This project demonstrates **Serde deserialization** in Rust by converting JSON data into strongly typed Rust structs.

---

## What This Example Covers

- Using `serde` with `Deserialize`
- Converting JSON strings into Rust structs
- Handling nested JSON objects
- Round-trip: Rust ➜ JSON ➜ Rust

---

## Data Model

The example uses three structs:

- `Address`
- `Payment`
- `Order` (contains `Address` and `Payment`)

All structs derive both `Serialize` and `Deserialize`.

---

## Dependencies

Add this to `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```
---

## How it works?
-  create and `order` struct in rust
-  Serialize it into `json`
-  Deserialize the `JSON` back into an `Order`
-  Deserialize a raw string into and `Order`

## Run the program
- `cargo run`

--- 


## Output

```text
Deserialized Order:
Order {
    order_id: "ORD101",
    customer_name: "Meet",
    address: Address {
        city: "Ahmedabad",
        state: "Gujarat",
        country: "India",
    },
    payment: Payment {
        method: "UPI",
        transaction_id: "TXN9001",
        amount: 2499.99,
    },
    total_items: 3,
    is_delivered: false,
    platform: "Amazon",
}

Deserialized Struct:
Order {
    order_id: "ORD101",
    customer_name: "Meet",
    address: Address {
        city: "Ahmedabad",
        state: "Gujarat",
        country: "India",
    },
    payment: Payment {
        method: "Card",
        transaction_id: "TXN777",
        amount: 4999.0,
    },
    total_items: 5,
    is_delivered: true,
    platform: "Flipkart",
}
