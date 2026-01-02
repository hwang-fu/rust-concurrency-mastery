//! Pitfall: Poisoned Mutex
//!
//! When a thread panics while holding a lock, the mutex becomes "poisoned".
//! Subsequent lock attempts return `Err(PoisonError)`.
//!
//! In production, prevent panics in locked sections rather than recovering from poison.
//! Ignoring poison should be a conscious choice, not a default.

use std::{
    sync::{Arc, Mutex},
    thread,
};

/// Demonstrates a mutex becoming poisoned after a panic.
pub fn demo_poisoned_mutex() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let cloned_data = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut guard = cloned_data.lock().unwrap();
        guard.push(4);
        panic!("Oops! Thread panicked while holding the lock!");
    });

    let _ = handle.join();

    // Try to lock again
    match data.lock() {
        Ok(guard) => println!("Got lock: {:?}", *guard),
        Err(poisoned) => {
            println!("Mutex is poisoned!");
            println!("Error: {}", poisoned);
        }
    }
}

/// Recovery option 1: Use `into_inner()` to get the data anyway.
/// This consumes the mutex entirely.
pub fn demo_recovery_into_inner() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let cloned_data = Arc::clone(&data);
    let _ = thread::spawn(move || {
        let mut guard = cloned_data.lock().unwrap();
        guard.push(4);
        panic!("Panic!");
    })
    .join();

    // Unwrap the Arc (only works if we're the only reference)
    match Arc::try_unwrap(data) {
        Ok(mutex) => {
            // into_inner() works even if poisoned
            let inner = mutex.into_inner().unwrap_or_else(|e| e.into_inner());
            println!("Recovered data: {:?}", inner);
        }
        Err(_) => println!("Cannot unwrap Arc - other references exist"),
    }
}

/// Recovery option 2: Use `PoisonError::into_inner()` to get the guard.
/// Data may be in an inconsistent state!
pub fn demo_recovery_get_ref() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let cloned_data = Arc::clone(&data);
    let _ = thread::spawn(move || {
        let mut guard = cloned_data.lock().unwrap();
        guard.push(4);
        panic!("Panic!");
    })
    .join();

    // Get the guard from the poisoned mutex
    match data.lock() {
        Ok(guard) => println!("Data: {:?}", *guard),
        Err(poisoned) => {
            println!("Mutex poisoned, recovering anyway...");
            let guard = poisoned.into_inner();
            println!("Recovered data (may be inconsistent): {:?}", *guard);
            // Note: The value 4 was pushed before the panic!
        }
    }
}

/// Best practice: Use `unwrap_or_else` for simple recovery.
pub fn demo_simple_recovery() {
    let data = Arc::new(Mutex::new(0));

    let cloned_data = Arc::clone(&data);
    let _ = thread::spawn(move || {
        let mut guard = cloned_data.lock().unwrap();
        *guard += 1;
        panic!("Panic!");
    })
    .join();

    // Simple pattern: ignore poison, get the data
    let guard = data.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    println!("Value (ignoring poison): {}", *guard);
}
