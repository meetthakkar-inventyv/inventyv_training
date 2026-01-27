use serde::{Serialize};

#[derive(Debug,Clone,Serialize)]
struct Address {
    city: String,
    state: String,
    country: String,
}

#[derive(Debug, Clone,Serialize)]
struct Payment {
    method: String,
    transaction_id: String,
    amount: f64,
}

#[derive(Debug, Clone,Serialize)]
struct Order {
    order_id: String,
    customer_name: String,
    address: Address, //Address Struct
    payment: Payment, //Payment Struct
    total_items: u16,
    is_delivered: bool,
    platform: String,
}

fn main(){
    let order = Order {
        order_id: "ORD101".to_string(),
        customer_name: "Meet".to_string(),
        address: Address {
            city: "Ahmedabad".to_string(),
            state: "Gujarat".to_string(),
            country: "India".to_string(),
        },
        payment: Payment {
            method: "UPI".to_string(),
            transaction_id: "TXN9001".to_string(),
            amount: 2499.99,
        },
        total_items: 3,
        is_delivered: false,
        platform: "Amazon".to_string(),
    };

    let serialized_order:String = serde_json::to_string_pretty(&order).unwrap();
    println!("Serialized Order:\n{}", serialized_order);
}