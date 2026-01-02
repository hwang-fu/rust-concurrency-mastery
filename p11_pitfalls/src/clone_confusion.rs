use std::{sync::Arc, thread};

/// WRONG: This moves `data` into the first thread.
/// The second spawn would fail to compile!
///
/// ```txt
/// use std::sync::Arc;
/// use std::thread;
///
/// let data = Arc::new(vec![1, 2, 3]);
///
/// // This MOVES data into the closure
/// thread::spawn(move || {
///     println!("{:?}", data);
/// });
///
/// // ERROR: data was moved above!
/// thread::spawn(move || {
///     println!("{:?}", data);
/// });
/// ```
pub fn demo_problem_explanantion() {
    println!("=== Clone Confusion ===");
    println!();
    println!("WRONG:");
    println!("  let data = Arc::new(vec![1, 2, 3]);");
    println!("  thread::spawn(move || println!(\"{{:?}}\", data));  // moves data");
    println!("  thread::spawn(move || println!(\"{{:?}}\", data));  // ERROR: already moved!");
    println!();
    println!("RIGHT:");
    println!("  let data = Arc::new(vec![1, 2, 3]);");
    println!("  let cloned_data = Arc::clone(&data);");
    println!("  thread::spawn(move || println!(\"{{:?}}\", cloned_data));");
    println!("  thread::spawn(move || println!(\"{{:?}}\", data));  // OK!");
}

/// CORRECT: Clone the Arc before moving into threads.
pub fn demo_fixed() {
    let data = Arc::new(vec![1, 2, 3]);

    let cloned_data = Arc::clone(&data);
    let h1 = thread::spawn(move || {
        println!("Thread 1: {:?}", cloned_data);
    });

    let cloned_data = Arc::clone(&data);
    let h2 = thread::spawn(move || {
        println!("Thread 2: {:?}", cloned_data);
    });

    // Original `data` still usable here!
    println!("Main: {:?}", data);

    h1.join().unwrap();
    h2.join().unwrap();
}

/// Tip: Clone inline for cleaner code in loops.
pub fn demo_loop_pattern() {
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for i in 0..3 {
        let cloned_data = Arc::clone(&data); // Clone BEFORE spawn
        handles.push(thread::spawn(move || {
            println!("Thread {}: {:?}", i, cloned_data);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final strong_count: {}", Arc::strong_count(&data));
}
