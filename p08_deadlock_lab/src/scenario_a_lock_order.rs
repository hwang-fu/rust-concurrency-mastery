use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

/// DEADLOCK: Two threads acquire locks in opposite order.
/// Thread 1: lock_a -> sleep -> lock_b
/// Thread 2: lock_b -> sleep -> lock_a
///
/// WARNING: This function will hang forever!
pub fn demo_deadlock() {
    let lock_a = Arc::new(Mutex::new("A"));
    let lock_b = Arc::new(Mutex::new("B"));

    let cloned_lock_a = Arc::clone(&lock_a);
    let cloned_lock_b = Arc::clone(&lock_b);

    let h1 = thread::spawn(move || {
        println!("Thread 1: waiting for lock A...");
        let _guard_a = cloned_lock_a.lock().unwrap();
        println!("Thread 1: acquired lock A");

        thread::sleep(Duration::from_millis(100));

        println!("Thread 1: waiting for lock B...");
        let _guard_b = cloned_lock_b.lock().unwrap();
        println!("Thread 1: acquired lock B");
    });

    let cloned_lock_a = Arc::clone(&lock_a);
    let cloned_lock_b = Arc::clone(&lock_b);

    let h2 = thread::spawn(move || {
        println!("Thread 2: waiting for lock B...");
        let _guard_b = cloned_lock_b.lock().unwrap();
        println!("Thread 2: acquired lock B");

        thread::sleep(Duration::from_millis(100));

        println!("Thread 2: waiting for lock A...");
        let _guard_a = cloned_lock_a.lock().unwrap();
        println!("Thread 2: acquired lock A");
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("Done!"); // Never reached
}

/// FIX: Both threads acquire locks in the same order (A before B).
pub fn demo_fixed() {
    let lock_a = Arc::new(Mutex::new("A"));
    let lock_b = Arc::new(Mutex::new("B"));

    let cloned_lock_a = Arc::clone(&lock_a);
    let cloned_lock_b = Arc::clone(&lock_b);

    let h1 = thread::spawn(move || {
        println!("Thread 1: waiting for lock A...");
        let _guard_a = cloned_lock_a.lock().unwrap();
        println!("Thread 1: acquired lock A");

        thread::sleep(Duration::from_millis(100));

        println!("Thread 1: waiting for lock B...");
        let _guard_b = cloned_lock_b.lock().unwrap();
        println!("Thread 1: acquired lock B");
    });

    let cloned_lock_a = Arc::clone(&lock_a);
    let cloned_lock_b = Arc::clone(&lock_b);

    let h2 = thread::spawn(move || {
        println!("Thread 2: waiting for lock A...");
        let _guard_a = cloned_lock_a.lock().unwrap();
        println!("Thread 2: acquired lock A");

        thread::sleep(Duration::from_millis(100));

        println!("Thread 2: waiting for lock B...");
        let _guard_b = cloned_lock_b.lock().unwrap();
        println!("Thread 2: acquired lock B");
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("Done!");
}
