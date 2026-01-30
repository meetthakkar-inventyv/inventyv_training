mod mutex;
mod rwlock;

fn main() {
    println!("Using Mutex....");
    print!("\n");
    mutex::print_stats();
    print!("\n");
    println!("Using RwLock....");
    print!("\n");
    rwlock::print_stats();
}
