# Serde Serialization 

This project demonstrates **Serde serialization** in Rust by converting a nested Rust struct into a JSON string.

---

## What This Example Covers

- Using `serde` with `derive`
- Serializing structs to JSON
- Handling nested structs
- Pretty-printing JSON output

---

## Data Model

The example uses three structs:

- `Address`
- `Payment`
- `Order` (contains `Address` and `Payment`)

All structs derive `Serialize`.

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
-  create and `order` struct with sample data
-  Serialize it using `serde_json::to_string_pretty`
- Print the resulting `JSON` string


## Run the program
- `cargo run`

## output
```json
Serialized Order:
{
  "order_id": "ORD101",
  "customer_name": "Meet",
  "address": {
    "city": "Ahmedabad",
    "state": "Gujarat",
    "country": "India"
  },
  "payment": {
    "method": "UPI",
    "transaction_id": "TXN9001",
    "amount": 2499.99
  },
  "total_items": 3,
  "is_delivered": false,
  "platform": "Amazon"
}
```