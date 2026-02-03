#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Address {
    city: String,
    state: String,
    country: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Payment {
    method: String,
    transaction_id: String,
    amount: f64,
}

#[derive(Debug, Clone)]
struct Order {
    order_id: String,
    customer_name: String,
    address: Address, //Address Struct
    payment: Payment, //Payment Struct
    total_items: u16,
    is_delivered: bool,
    platform: String,
}
#[allow(dead_code)]
impl Order {

    ///Setters
    fn set_order_id(&mut self, id: String) {
        self.order_id = id;
        self.print_after_set("set_customer_id");
    }

    fn set_customer_name(&mut self, name: String) {
        self.customer_name = name;
        self.print_after_set("set_customer_name");
    }


    fn set_city(&mut self, city: String) {
        self.address.city = city;
        self.print_after_set("set_customer_city");
    }

    fn set_state(&mut self, state: String) {
        self.address.state = state;
        self.print_after_set("set_customer_state");
    }

    fn set_country(&mut self, country: String) {
        self.address.country = country;
        self.print_after_set("set_customer_country");
    }

    fn set_payment_method(&mut self, method: String) {
        self.payment.method = method;
        self.print_after_set("set_payment_method");
    }

    fn set_transaction_id(&mut self, txn_id: String) {
        self.payment.transaction_id = txn_id;
        self.print_after_set("set_payment_transaction_id");
    }

    fn set_amount(&mut self, amount: f64) {
        self.payment.amount = amount;
        self.print_after_set("set_payment_amount");
    }

    fn set_total_items(&mut self, items: u16) {
        self.total_items = items;
        self.print_after_set("set_total_items");
    }

    fn set_is_delivered(&mut self, delivered: bool) {
        self.is_delivered = delivered;
        self.print_after_set("set_is_delivered");
    }

    fn set_platform(&mut self, platform: String) {
        self.platform = platform;
        self.print_after_set("set_platform");
    }

    fn set_full_order(&mut self, order_id:String, customer_name:String, address:Address, payment:Payment, total_items:u16, is_delivered:bool, platform:String)  {
        self.order_id = order_id;
        self.customer_name = customer_name;
        self.address = address;
        self.payment = payment;
        self.total_items = total_items;
        self.is_delivered = is_delivered;
        self.platform = platform;
        self.print_after_set("set_full_order");
    }

    fn set_full_order_wo_self(order_id:String, customer_name:String, address:Address, payment:Payment, total_items:u16, is_delivered:bool, platform:String) -> Self  {
        Self {
            order_id,
            customer_name,
            address,
            payment,
            total_items,
            is_delivered,
            platform
        }
    }

    ///Getters
    fn get_order_id(&self) -> String {
        self.order_id.clone()
    }

    fn get_customer_name(&self) -> String {
        self.customer_name.clone()
    }

    fn get_city(&self) -> String {
        self.address.city.clone()
    }

    fn get_state(&self) -> String {
        self.address.state.clone()
    }

    fn get_country(&self) -> String {
        self.address.country.clone()
    }

    fn get_payment_method(&self) -> String {
        self.payment.method.clone()
    }

    fn get_transaction_id(&self) -> String {
        self.payment.transaction_id.clone()
    }

    fn get_amount(&self) -> f64 {
        self.payment.amount
    }

    fn get_total_items(&self) -> u16 {
        self.total_items
    }


    fn get_delivery_status(&self) -> String {
        if self.is_delivered {
            "Order Delivered".to_string()
        } else {
            "Order Pending".to_string()
        }
    }

    fn get_platform(&self) -> String {
        self.platform.clone()
    }

    fn get_full_order(&self) -> String {
        format!(
            "Order ID : {}\nCustomer : {}\nCity : {}\nState : {}\nCountry : {}\nPayment Method : {}\nTransaction ID : {}\nAmount : {}\nTotal Items : {}\nDelivered : {}\nPlatform : {}",
            self.order_id,
            self.customer_name,
            self.address.city,
            self.address.state,
            self.address.country,
            self.payment.method,
            self.payment.transaction_id,
            self.payment.amount,
            self.total_items,
            self.is_delivered,
            self.platform
        )
    }

    fn get_full_order_wo_self(
        order_id: String,
        customer: String,
        address: Address,
        payment: Payment,
        items: u16,
        delivered: bool,
        platform: String,
    ) -> String {
        format!(
            "Order ID : {}\nCustomer : {}\nCity : {}\nState : {}\nCountry : {}\nPayment Method : {}\nTransaction ID : {}\nAmount : {}\nTotal Items : {}\nDelivered : {}\nPlatform : {}",
            order_id,
            customer,
            address.city,
            address.state,
            address.country,
            payment.method,
            payment.transaction_id,
            payment.amount,
            items,
            delivered,
            platform
        )
    }

     fn print_after_set(&self, called_from: &str) {
        println!(
            "\n--- After calling {} ---",
            called_from
        );

        println!(
            "\nUsing self (full struct):\n{:#?}",
            self
        );

        println!(
            "\nUsing &self (full struct):\n{:#?}",
            &self
        );
    }

}

fn main() {
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
    
    //Set individual fields
    let mut custom_order = order.clone();
    custom_order.set_is_delivered(true);
    custom_order.set_amount(2799.50);

    //Set full order with self
    let mut order = Order {
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

    order.set_full_order(
        "ORD202".to_string(),
        "Rahul".to_string(),
        Address {
            city: "Pune".to_string(),
            state: "Maharashtra".to_string(),
            country: "India".to_string(),
        },
        Payment {
            method: "Card".to_string(),
            transaction_id: "TXN777".to_string(),
            amount: 4999.0,
        },
        5,
        true,
        "Flipkart".to_string()
    );

    
}
