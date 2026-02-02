use serde::{Deserialize};
use serde::{Serialize};
#[allow(dead_code)]
#[derive(Debug,Clone,Deserialize,Serialize)]
struct Address {
    city: String,
    state: String,
    country: String,
}

#[derive(Debug, Clone,Deserialize,Serialize)]
struct Payment {
    method: String,
    transaction_id: String,
    amount: f64,
}

#[derive(Debug, Clone,Deserialize,Serialize)]
struct Order {
    order_id: String,
    customer_name: String,
    address: Address, //Address Struct
    payment: Payment, //Payment Struct
    total_items: u16,
    is_delivered: bool,
    platform: String,
}

pub fn run() {

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
    let deserialized_order: Order = serde_json::from_str(&serialized_order).unwrap();

    println!("Deserialized Order:\n{:#?}", deserialized_order);

    //Deserializing using raw string 
    let raw_json_string = r#"{
         "order_id": "ORD101",
        "customer_name": "Meet",
        "address": {
            "city": "Ahmedabad",
            "state": "Gujarat",
            "country": "India"
        },
        "payment": {
            "method": "Card",
            "transaction_id": "TXN777",
            "amount": 4999.0
        },
        "total_items": 5,
        "is_delivered": true,
        "platform": "Flipkart"
    }"#;

    let order : Order  = serde_json::from_str(raw_json_string).unwrap();
    println!("Deserialized Struct:\n{:#?}", order);

}