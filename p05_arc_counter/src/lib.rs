use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    thread,
};

pub fn demo_arc_no_mutation() {
    let cnt = Arc::new(0_u64);

    let _cloned_cnt = Arc::clone(&cnt);
    thread::spawn(move || {
        // *_cloned_cnt += 1
    });
}

/// Increments a shared counter using AtomicU64 (lock-free).
pub fn counter_with_atomic(num_threads: usize, increments_per_thread: usize) -> u64 {
    let counter = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for _ in 1..=num_threads {
        let cloned_counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 1..=increments_per_thread {
                cloned_counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter.load(Ordering::SeqCst)
}

/// Increments a shared counter using Mutex (locking).
pub fn counter_with_mutex(num_threads: usize, increments_per_thread: usize) -> u64 {
    let counter = Arc::new(Mutex::new(0_u64));

    let mut handles = vec![];

    for _ in 1..=num_threads {
        let cloned_counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 1..=increments_per_thread {
                let mut guard = cloned_counter.lock().unwrap();
                *guard += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    *(counter.lock().unwrap())
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_atomic_counter() {
        let result = counter_with_atomic(10, 1000);
        assert_eq!(result, 10_000);
    }

    #[test]
    fn test_mutex_counter() {
        let result = counter_with_mutex(10, 1000);
        assert_eq!(result, 10_000);
    }

    #[test]
    fn benchmark_comparison() {
        let num_threads = 10;
        let increments = 100_000;

        // Benchmark Atomic
        let start = Instant::now();
        let atomic_result = counter_with_atomic(num_threads, increments);
        let atomic_duration = start.elapsed();

        // Benchmark Mutex
        let start = Instant::now();
        let mutex_result = counter_with_mutex(num_threads, increments);
        let mutex_duration = start.elapsed();

        // Both should give same result
        assert_eq!(atomic_result, mutex_result);
        assert_eq!(atomic_result, (num_threads * increments) as u64);

        println!("\n=== Benchmark Results ===");
        println!(
            "Threads: {}, Increments per thread: {}",
            num_threads, increments
        );
        println!("Atomic: {:?}", atomic_duration);
        println!("Mutex:  {:?}", mutex_duration);
        println!(
            "Atomic is {:.2}x faster",
            mutex_duration.as_nanos() as f64 / atomic_duration.as_nanos() as f64
        );
    }
}
