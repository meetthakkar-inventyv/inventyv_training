use crate::modules::req_track_mutex;
use crate::modules::req_track_rwlock;

pub fn run() {
    println!("Using Mutex....");
    print!("\n");
    req_track_mutex::print_stats();
    print!("\n");
    println!("Using RwLock....");
    print!("\n");
    req_track_rwlock::print_stats();
}
