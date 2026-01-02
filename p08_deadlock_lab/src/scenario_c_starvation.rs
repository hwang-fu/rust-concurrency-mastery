//! Scenario C: RwLock Writer Starvation
//!
//! When readers continuously hold the lock, writers may wait forever.
//! The behavior of std::sync::RwLock is OS-dependent.

use std::{
    sync::{Arc, RwLock},
    thread::{self},
    time::{Duration, Instant},
};

pub fn demo_potential_starvation() {
    let data = Arc::new(RwLock::new(0));

    let start = Instant::now();

    let mut handles = vec![];

    // Spawn 10 aggressive readers
    for reader_id in 0..10 {
        let cloned_data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let guard = cloned_data.read().unwrap();
                thread::sleep(Duration::from_millis(5));
                let _ = *guard;
            }
            println!("Reader {} finished at {:?}", reader_id, start.elapsed());
        }));
    }

    // Spawn 1 writer that tries to get in
    let cloned_data = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        // Small delay to let readers start
        thread::sleep(Duration::from_millis(10));

        println!("Writer: attempting to acquire write lock...");
        let writer_start = Instant::now();
        let mut guard = cloned_data.write().unwrap();
        let wait_time = writer_start.elapsed();

        *guard += 1;
        println!(
            "Writer: acquired lock after {:?}, value = {}",
            wait_time, *guard
        );
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *data.read().unwrap());
}

/// Discussion: Fairer alternatives
///
/// 1. `parking_lot::RwLock` - Provides fairer scheduling
/// 2. Use a `Mutex` if writes are frequent
/// 3. Batch reads to reduce lock contention
/// 4. Use read-copy-update (RCU) patterns for read-heavy workloads
pub fn discussion() {
    println!("=== RwLock Fairness Discussion ===");
    println!();
    println!("std::sync::RwLock fairness is OS-dependent:");
    println!("  - Linux (glibc): Writer-preferring");
    println!("  - macOS: Can starve writers");
    println!("  - Windows: Usually fair");
    println!();
    println!("Alternatives for better fairness:");
    println!("  1. parking_lot::RwLock - consistent cross-platform behavior");
    println!("  2. Use Mutex if writes are frequent");
    println!("  3. Batch reads to reduce contention");
    println!("  4. Consider RCU patterns for read-heavy workloads");
}
