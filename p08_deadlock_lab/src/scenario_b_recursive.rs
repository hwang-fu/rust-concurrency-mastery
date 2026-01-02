//! Scenario B: Recursive Lock Deadlock
//!
//! Rust's std::sync::Mutex is not reentrant. If a thread tries to lock a mutex it already holds, it deadlocks on itself.
//!
//! A thread tries to acquire the same mutex twice, causing self-deadlock.
//! Rust's std::sync::Mutex is NOT reentrant.

use std::sync::{Arc, Mutex};

pub fn demo_deadlock() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let guard = data.lock().unwrap();
    println!("First lock acquired, data: {:?}", *guard);

    // Oops! Trying to lock again while still holding the lock
    println!("Attempting to acquire lock again...");
    let _guard2 = data.lock().unwrap(); // DEADLOCK!
    println!("This line never executes");
}

pub fn demo_deadlock_nested() {
    let counter = Arc::new(Mutex::new(0));

    fn inner_op(counter: &Arc<Mutex<i32>>) {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
        println!("Inner: counter = {}", *guard);
    }

    let mut guard = counter.lock().unwrap();
    *guard += 10;
    println!("Outer: counter = {}", *guard);

    drop(guard); // COMMENT THIS LINE TO SEE DEADLOCK
    inner_op(&counter);
}

/// FIX 1: Release the lock before calling functions that need it.
pub fn demo_fixed_drop_early() {
    let counter = Arc::new(Mutex::new(0));

    fn inner_operation(counter: &Arc<Mutex<i32>>) {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
        println!("Inner: counter = {}", *guard);
    }

    {
        let mut guard = counter.lock().unwrap();
        *guard += 10;
        println!("Outer: counter = {}", *guard);
    } // Lock released here

    inner_operation(&counter); // Safe to call now
    println!("Done!");
}

/// FIX 2: Pass the guard instead of re-acquiring the lock.
pub fn demo_fixed_pass_guard() {
    let counter = Arc::new(Mutex::new(0));

    fn inner_operation(guard: &mut i32) {
        *guard += 1;
        println!("Inner: counter = {}", *guard);
    }

    let mut guard = counter.lock().unwrap();
    *guard += 10;
    println!("Outer: counter = {}", *guard);

    inner_operation(&mut guard); // Pass the guard, not the mutex (guard auto derefs, you can also write &mut *guard tho)
    println!("Done! Final: {}", *guard);
}
