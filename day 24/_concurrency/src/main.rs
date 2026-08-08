// Day 24 — Concurrency: threads, Arc<Mutex<T>>, and channels
// The payoff of Week 1: the borrow checker prevents data races AT COMPILE TIME.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    // ===============================================================
    // 1. Basic threads. `move` gives the closure OWNERSHIP of what it uses.
    // ===============================================================
    println!("--- basic threads ---");
    let mut handles = vec![];

    for id in 1..=3 {
        // `move` moves `id` INTO the thread — the thread owns its copy.
        let handle = thread::spawn(move || {
            println!("worker {} is running", id);
        });
        handles.push(handle);
    }

    // join() waits for each thread to finish before main continues.
    for handle in handles {
        handle.join().unwrap();
    }

    // ===============================================================
    // 2. Arc<Mutex<T>> — shared, mutable state across threads.
    //    Arc = thread-safe shared ownership (Day 17's Rc, but atomic).
    //    Mutex = only one thread touches the inside at a time.
    // ===============================================================
    println!("\n--- Arc<Mutex<T>> shared counter ---");

    // A counter shared by all threads.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        // Clone the Arc: another owner (a handle to the SAME counter).
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock() gives exclusive access; it unlocks when `num` drops.
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // All 5 threads incremented the SAME counter, safely. Result: 5.
    println!("Final count: {}", *counter.lock().unwrap());

    // ===============================================================
    // 3. Channels — threads communicate by SENDING MESSAGES.
    //    "Don't communicate by sharing memory; share memory by communicating."
    //    mpsc = multiple producer, single consumer.
    // ===============================================================
    println!("\n--- channels ---");

    let (tx, rx) = mpsc::channel(); // tx = transmitter, rx = receiver

    for id in 1..=3 {
        let tx = tx.clone(); // each thread gets its own sender
        thread::spawn(move || {
            let message = format!("report from worker {}", id);
            tx.send(message).unwrap(); // send down the channel
        });
    }
    drop(tx); // drop the ORIGINAL sender so the receiver knows when all are done

    // rx acts like an iterator — yields each message until all senders are gone.
    for received in rx {
        println!("main received: {}", received);
    }

    println!("\nAll workers reported in.");
}