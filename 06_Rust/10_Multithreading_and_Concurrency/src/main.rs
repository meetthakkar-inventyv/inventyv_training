use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct MultiThread {
    id: i32,
    record_added_time: String,
    thread_id: String,
}

fn main() {

    let shared_data: Arc<Mutex<Vec<MultiThread>>> =
        Arc::new(Mutex::new(Vec::new()));

    let global_counter = Arc::new(AtomicI32::new(1));

    // ---------------- Thread 1: Record Creator ----------------
    let data_clone = Arc::clone(&shared_data);
    let counter_clone = Arc::clone(&global_counter);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));

            let id = counter_clone.fetch_add(1, Ordering::SeqCst);

            let now: DateTime<Local> = Local::now();
            let time_string = now.to_rfc3339();

            let new_record = MultiThread {
                id,
                record_added_time: time_string.clone(),
                thread_id: format!("T-{}", id),
            };

            let mut data = data_clone.lock().unwrap();
            data.push(new_record);

            println!("[CREATOR] Added record id={} at {}", id, time_string);
        }
    });

    // ---------------- Thread 2: State Printer ----------------
    let data_clone = Arc::clone(&shared_data);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));

            let data = data_clone.lock().unwrap();
            println!("[PRINTER] Current Records Count: {}", data.len());
            println!("[PRINTER] Records: {:?}", *data);
        }
    });

    // ---------------- Thread 3: Even Cleaner ----------------
    let data_clone = Arc::clone(&shared_data);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));

            let mut data = data_clone.lock().unwrap();

            data.retain(|record| {
                let stored_time =
                    DateTime::parse_from_rfc3339(&record.record_added_time)
                        .unwrap()
                        .with_timezone(&Local);

                let elapsed =
                    Local::now().signed_duration_since(stored_time);

                let should_remove =
                    record.id % 2 == 0 && elapsed.num_seconds() > 20;

                if should_remove {
                    println!(
                        "[EVEN CLEANER] Removing id={} elapsed={} seconds",
                        record.id,
                        elapsed.num_seconds()
                    );
                }

                !should_remove
            });
        }
    });

    // ---------------- Thread 4: Odd Cleaner ----------------
    let data_clone = Arc::clone(&shared_data);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));

            let mut data = data_clone.lock().unwrap();

            data.retain(|record| {
                let stored_time =
                    DateTime::parse_from_rfc3339(&record.record_added_time)
                        .unwrap()
                        .with_timezone(&Local);

                let elapsed =
                    Local::now().signed_duration_since(stored_time);

                let should_remove =
                    record.id % 2 != 0 && elapsed.num_seconds() > 20;

                if should_remove {
                    println!(
                        "[ODD CLEANER] Removing id={} elapsed={} seconds",
                        record.id,
                        elapsed.num_seconds()
                    );
                }

                !should_remove
            });
        }
    });

    // ---------------- Thread 5: Even Counter ----------------
    let data_clone = Arc::clone(&shared_data);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(7));

            let data = data_clone.lock().unwrap();
            let count =
                data.iter().filter(|r| r.id % 2 == 0).count();

            println!("[EVEN COUNTER] Even records count: {}", count);
        }
    });

    // ---------------- Thread 6: Odd Counter ----------------
    let data_clone = Arc::clone(&shared_data);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(7));

            let data = data_clone.lock().unwrap();
            let count =
                data.iter().filter(|r| r.id % 2 != 0).count();

            println!("[ODD COUNTER] Odd records count: {}", count);
        }
    });

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
