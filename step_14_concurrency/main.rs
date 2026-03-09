// ============================================================
// STEP 14: Concurrency
// ============================================================
// Run: rustc main.rs && ./main
//
// Rust's ownership system makes concurrent programming safer.
// "Fearless concurrency" — many bugs caught at compile time.
//
// Key tools:
// - std::thread: OS threads
// - std::sync::Mutex: mutual exclusion
// - std::sync::Arc: atomic reference counting (thread-safe Rc)
// - std::sync::mpsc: message passing channels
// ============================================================

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // =========================
    // 14.1 Spawning Threads
    // =========================

    println!("--- Spawning Threads ---");

    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  [spawned] count: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("  [main] count: {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    // Wait for the spawned thread to finish:
    handle.join().unwrap();
    println!("Both threads done.\n");

    // =========================
    // 14.2 Moving Data into Threads
    // =========================

    println!("--- Move into Threads ---");

    let data = vec![1, 2, 3, 4, 5];

    // `move` transfers ownership of `data` into the thread closure.
    let handle = thread::spawn(move || {
        println!("  Thread got data: {:?}", data);
        let sum: i32 = data.iter().sum();
        sum // return value from thread
    });

    // data is no longer available here (it was moved)
    let result = handle.join().unwrap();
    println!("  Thread returned: {}\n", result);

    // =========================
    // 14.3 Multiple Threads
    // =========================

    println!("--- Multiple Threads ---");

    let mut handles = vec![];

    for id in 0..5 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 * id as u64));
            println!("  Thread {} finished", id);
            id * id
        });
        handles.push(handle);
    }

    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("  Results: {:?}\n", results);

    // =========================
    // 14.4 Message Passing with Channels (mpsc)
    // =========================

    println!("--- Channels (mpsc) ---");

    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["hello", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        // tx is dropped here, closing the channel
    });

    // Receive messages (blocks until message arrives):
    for received in rx {
        println!("  Got: {}", received);
    }
    println!();

    // =========================
    // 14.5 Multiple Producers
    // =========================

    println!("--- Multiple Producers ---");

    let (tx, rx) = mpsc::channel();

    for id in 0..3 {
        let tx_clone = tx.clone(); // clone the sender for each thread
        thread::spawn(move || {
            for i in 0..3 {
                tx_clone
                    .send(format!("Thread {} msg {}", id, i))
                    .unwrap();
                thread::sleep(Duration::from_millis(30));
            }
        });
    }
    drop(tx); // drop the original sender so rx knows when all are done

    for msg in rx {
        println!("  {}", msg);
    }
    println!();

    // =========================
    // 14.6 Mutex<T> — Shared Mutable State
    // =========================

    println!("--- Mutex ---");

    // Mutex provides mutual exclusion — only one thread can access at a time.
    let counter = Mutex::new(0);

    // Single-threaded example first:
    {
        let mut num = counter.lock().unwrap(); // acquire lock
        *num += 1;
        println!("  Counter: {}", num);
    } // lock released when `num` goes out of scope

    println!("  Counter: {:?}\n", counter);

    // =========================
    // 14.7 Arc<Mutex<T>> — Thread-Safe Shared State
    // =========================

    println!("--- Arc<Mutex<T>> ---");

    // Arc = Atomic Rc (thread-safe reference counting)
    // Rc is NOT thread-safe, so we use Arc for multi-threaded sharing.

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Final counter: {}\n", *counter.lock().unwrap());

    // =========================
    // 14.8 Practical: Parallel Map
    // =========================

    println!("--- Parallel Map ---");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let results = parallel_map(&data, |&x| {
        thread::sleep(Duration::from_millis(10)); // simulate work
        x * x
    });
    println!("  Input:  {:?}", data);
    println!("  Output: {:?}\n", results);

    // =========================
    // 14.9 Practical: Producer-Consumer
    // =========================

    println!("--- Producer-Consumer ---");

    let (tx, rx) = mpsc::channel();

    // Producer thread
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i * 10).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    // Consumer: process items as they arrive
    let consumer = thread::spawn(move || {
        let mut total = 0;
        for val in rx {
            println!("  Processing: {}", val);
            total += val;
        }
        total
    });

    producer.join().unwrap();
    let total = consumer.join().unwrap();
    println!("  Total processed: {}\n", total);

    // =========================
    // 14.10 Thread-Safe Data Structures
    // =========================

    println!("--- Thread-Safe Accumulator ---");

    let accumulator = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let acc = Arc::clone(&accumulator);
        handles.push(thread::spawn(move || {
            let result = i * i;
            acc.lock().unwrap().push((i, result));
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let mut results = accumulator.lock().unwrap().clone();
    results.sort_by_key(|&(i, _)| i);
    println!("  Results: {:?}", results);

    println!("\n--- Step 14 Complete! ---");
    println!("Next: step_15 — Async/Await");
}

// ============================================================
// Helper Functions
// ============================================================

fn parallel_map<T, R, F>(data: &[T], f: F) -> Vec<R>
where
    T: Sync + Send + 'static + Clone,
    R: Send + 'static,
    F: Fn(&T) -> R + Send + Sync + 'static + Clone,
{
    let mut handles = vec![];

    for item in data {
        let item = item.clone();
        let f = f.clone();
        handles.push(thread::spawn(move || f(&item)));
    }

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

// ============================================================
// EXERCISES:
// 1. Spawn 5 threads that each compute factorial(n) for n=1..5.
//    Collect results using channels.
// 2. Use Arc<Mutex<HashMap>> to build a thread-safe counter.
//    Multiple threads increment counts for different keys.
// 3. Implement a simple thread pool that processes a queue of tasks.
// 4. Use channels to implement a pipeline: generator -> transformer -> printer.
// ============================================================
