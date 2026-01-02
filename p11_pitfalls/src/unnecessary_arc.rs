//! Pitfall: Unnecessary Arc
//!
//! Using `Arc` when the data doesn't actually need shared ownership.
//! Simpler alternatives: owned data, scoped threads, or channels.

use std::sync::Arc;
use std::thread;

/// OVERKILL: Using Arc when data could just be moved.
pub fn demo_unnecessary() {
    let data = Arc::new(vec![1, 2, 3]);

    // Only one thread needs the data - Arc is overkill!
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("Data: {:?}", data_clone);
    });

    handle.join().unwrap();
    // `data` is never used again - we could have just moved it!
}

/// BETTER: Just move the data if only one thread needs it.
pub fn demo_just_move() {
    let data = vec![1, 2, 3];

    // Move directly - no Arc needed!
    let handle = thread::spawn(move || {
        println!("Data: {:?}", data);
    });

    handle.join().unwrap();
}

/// BETTER: Use scoped threads for borrowing without Arc.
pub fn demo_scoped_threads() {
    let data = vec![1, 2, 3];

    // Scoped threads can borrow - no Arc or clone needed!
    thread::scope(|s| {
        s.spawn(|| {
            println!("Thread 1: {:?}", data);
        });
        s.spawn(|| {
            println!("Thread 2: {:?}", data);
        });
    });

    // `data` is still usable here!
    println!("Main: {:?}", data);
}

/// BETTER: Use channels to transfer ownership.
pub fn demo_channels() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let data = vec![1, 2, 3];
        tx.send(data).unwrap(); // Move data through channel
    });

    let received = rx.recv().unwrap();
    println!("Received: {:?}", received);
}

pub fn guidelines() {
    println!("=== When to Use What ===");
    println!();
    println!("Use OWNED data (move):");
    println!("  - Only one thread needs the data");
    println!();
    println!("Use SCOPED THREADS:");
    println!("  - Multiple threads need to READ the same data");
    println!("  - Threads finish before the data goes out of scope");
    println!();
    println!("Use CHANNELS:");
    println!("  - Transferring ownership between threads");
    println!("  - Producer-consumer patterns");
    println!();
    println!("Use ARC:");
    println!("  - Multiple threads need SHARED OWNERSHIP");
    println!("  - Threads outlive the creating scope");
    println!("  - Data lifetime is dynamic/unpredictable");
}
