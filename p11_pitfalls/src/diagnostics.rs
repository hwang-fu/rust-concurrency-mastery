//! Diagnostic Techniques
//!
//! Tools and patterns for debugging concurrency issues.

use std::sync::{Arc, Weak};

/// Use strong_count and weak_count to debug reference issues.
pub fn demo_reference_counting() {
    println!("=== Reference Counting Diagnostics ===");

    let data = Arc::new("shared data");
    println!(
        "After creation: strong={}, weak={}",
        Arc::strong_count(&data),
        Arc::weak_count(&data)
    );

    let clone1 = Arc::clone(&data);
    println!(
        "After clone1: strong={}, weak={}",
        Arc::strong_count(&data),
        Arc::weak_count(&data)
    );

    let weak1 = Arc::downgrade(&data);
    println!(
        "After weak1: strong={}, weak={}",
        Arc::strong_count(&data),
        Arc::weak_count(&data)
    );

    drop(clone1);
    println!(
        "After drop clone1: strong={}, weak={}",
        Arc::strong_count(&data),
        Arc::weak_count(&data)
    );

    drop(weak1);
    println!(
        "After drop weak1: strong={}, weak={}",
        Arc::strong_count(&data),
        Arc::weak_count(&data)
    );
}

/// Use Drop to trace destruction order.
pub fn demo_drop_tracing() {
    println!("\n=== Drop Tracing ===");

    struct Traced {
        name: String,
    }

    impl Drop for Traced {
        fn drop(&mut self) {
            println!("  Dropping: {}", self.name);
        }
    }

    println!("Creating objects...");
    let _a = Traced {
        name: "A (created first)".into(),
    };
    let _b = Traced {
        name: "B (created second)".into(),
    };
    let _c = Traced {
        name: "C (created third)".into(),
    };

    println!("Leaving scope (drops in reverse order)...");
    // C, B, A will be dropped in that order
}

/// Detect unexpected Arc retention.
pub fn demo_leak_detection() {
    println!("\n=== Leak Detection ===");

    let data = Arc::new(vec![1, 2, 3]);

    // Simulate some operations that might retain references
    let _stored: Vec<Arc<Vec<i32>>> = (0..3).map(|_| Arc::clone(&data)).collect();

    let count = Arc::strong_count(&data);
    println!("strong_count = {} (expected: 4)", count);

    if count > 4 {
        println!("WARNING: Unexpected references detected!");
    }

    // Check if we can unwrap (only reference left)
    drop(_stored);
    match Arc::try_unwrap(data) {
        Ok(inner) => println!("Successfully unwrapped: {:?}", inner),
        Err(arc) => println!("Cannot unwrap, strong_count = {}", Arc::strong_count(&arc)),
    }
}

/// Check if a Weak reference is still valid.
pub fn demo_weak_validity() {
    println!("\n=== Weak Reference Validity ===");

    let weak: Weak<String>;
    {
        let strong = Arc::new(String::from("I exist!"));
        weak = Arc::downgrade(&strong);

        println!("Inside scope: upgrade = {:?}", weak.upgrade());
    }

    println!("Outside scope: upgrade = {:?}", weak.upgrade());
    println!("Weak::strong_count = {}", weak.strong_count());
}

pub fn summary() {
    println!("\n=== Diagnostic Techniques Summary ===");
    println!();
    println!("1. Arc::strong_count() / weak_count()");
    println!("   - Track reference counts at key points");
    println!("   - Detect unexpected retention or leaks");
    println!();
    println!("2. impl Drop with println!");
    println!("   - Trace destruction order");
    println!("   - Verify objects are actually dropped");
    println!();
    println!("3. Arc::try_unwrap()");
    println!("   - Check if you're the only owner");
    println!("   - Detect unexpected shared references");
    println!();
    println!("4. Weak::upgrade()");
    println!("   - Check if referenced data still exists");
    println!("   - Returns None if all strong refs dropped");
}
