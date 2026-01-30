use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

enum Request {
    Get{endpoint : String},
    Post{endpoint : String, data_size: u32},
    Delete {id: u32}
}

static TOTAL_COUNT: AtomicUsize = AtomicUsize::new(0);

static GET_COUNT: RwLock<u32> = RwLock::new(0);
static POST_COUNT: RwLock<u32> = RwLock::new(0);
static DELETE_COUNT: RwLock<u32> = RwLock::new(0);

fn handle_request(req: Request) -> String {
    TOTAL_COUNT.fetch_add(1,Ordering::SeqCst);

    match req{
        Request::Get{endpoint} => {
            *GET_COUNT.write().unwrap() += 1;
            format!("GET request to {}", endpoint)
        }
        Request::Post{endpoint, data_size} => {
            *POST_COUNT.write().unwrap() += 1;
            format!("POST request to {} with data size {}", endpoint, data_size)
        }
        Request::Delete{id} => {
            *DELETE_COUNT.write().unwrap() += 1;
            format!("DELETE request with id {}", id)
        }
    }
}

pub fn print_stats(){
     println!(
        "{}",
        handle_request(Request::Get{endpoint: String::from("/home")})
    );
     println!(
        "{}",
        handle_request(Request::Get{endpoint: String::from("/home")})
    );
    println!(
        "{}",
        handle_request(Request::Post{endpoint: String::from("/home"), data_size: 256})
    );
    println!(
        "{}",
        handle_request(Request::Post{endpoint: String::from("/home"), data_size: 256})
    );
   
    println!("{}", handle_request(Request::Delete{id: 1}));
    println!("{}", handle_request(Request::Delete{id: 1}));
    println!("GET count: {}", *GET_COUNT.read().unwrap());
    println!("POST count: {}", *POST_COUNT.read().unwrap());
    println!("DELETE count: {}", *DELETE_COUNT.read().unwrap());
    
    println!(
        "TOTAL count: {}",
        TOTAL_COUNT.load(Ordering::SeqCst)
    );
}