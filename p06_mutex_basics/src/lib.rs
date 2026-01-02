use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

/// A thread-safe log collector.
/// The Log struct itself is just a wrapper around Vec<String>.
/// Thread-safety comes from wrapping it in Arc<Mutex<Log>>.
pub struct Log {
    entries: Vec<String>,
}

impl Log {
    pub fn new() -> Self {
        Log {
            entries: Vec::new(),
        }
    }

    pub fn append(&mut self, message: String) {
        self.entries.push(message)
    }

    pub fn getall(&self) -> &[String] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}

pub fn demo_threaded_logging() {
    let log = Arc::new(Mutex::new(Log::new()));

    let mut handles = vec![];

    for thread_id in 1..=5 {
        let cloned_log = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            for i in 1..=10 {
                let mut guard = cloned_log.lock().unwrap();
                guard.append(format!("Thread {} - message {}", thread_id, i));
                // guard is dropped here, releasing the lock
            }
        }));
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    let final_log = log.lock().unwrap();
    println!("Total log entries: {}", final_log.len());

    println!("\nFirst 10 entries (notice thread interleaving):");
    for entry in final_log.getall().iter().take(10) {
        println!("  {}", entry)
    }
}

pub fn log_count(log: &Arc<Mutex<Log>>) -> usize {
    log.lock().unwrap().len()
}

/// Demonstrates the effect of holding a lock too long.
/// Other threads are blocked waiting for the lock.
pub fn demo_long_lock() {
    let log = Arc::new(Mutex::new(Log::new()));
    let start = Instant::now();

    let mut handles = vec![];

    for thread_id in 1..=3 {
        let cloned_log = Arc::clone(&log);
        let cloned_start = start.clone();

        handles.push(thread::spawn(move || {
            // Acquire the lock
            let mut guard = cloned_log.lock().unwrap();
            let acquired_at = cloned_start.elapsed().as_millis();

            // Simulate slow work WHILE HOLDING THE LOCK (bad practice!)
            thread::sleep(Duration::from_millis(100));

            guard.append(format!(
                "Thread {} (lock acquired at {}ms)",
                thread_id, acquired_at
            ));

            // Guard dropped here - lock released
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n=== Long Lock Demo ===");
    println!("Notice how threads acquire locks sequentially (~0ms, ~100ms, ~200ms):");
    for entry in log.lock().unwrap().getall() {
        println!("  {}", entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threaded_logging() {
        let log = Arc::new(Mutex::new(Log::new()));
        let mut handles = vec![];

        for thread_id in 1..=5 {
            let cloned_log = Arc::clone(&log);
            handles.push(thread::spawn(move || {
                for i in 1..=10 {
                    let mut guard = cloned_log.lock().unwrap();
                    guard.append(format!("Thread {} - message {}", thread_id, i));
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // 5 threads × 10 messages = 50 total
        let final_log = log.lock().unwrap();
        assert_eq!(final_log.len(), 50);
    }

    #[test]
    fn test_log_count_helper() {
        let log = Arc::new(Mutex::new(Log::new()));
        assert_eq!(log_count(&log), 0);

        log.lock().unwrap().append("test message...".to_string());
        assert_eq!(log_count(&log), 1);
    }

    #[test]
    fn test_long_lock_blocks_threads() {
        let log = Arc::new(Mutex::new(Log::new()));
        let start = Instant::now();

        let mut handles = vec![];

        for thread_id in 1..=3 {
            let cloned_log = Arc::clone(&log);

            handles.push(thread::spawn(move || {
                let mut guard = cloned_log.lock().unwrap();
                thread::sleep(Duration::from_millis(50));
                guard.append(format!("Thread {}", thread_id));
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let elapsed = start.elapsed().as_millis();

        // 3 threads × 50ms each = ~150ms minimum (sequential, not parallel)
        assert!(
            elapsed >= 140,
            "Expected ~150ms, got {}ms - threads should block",
            elapsed
        );
        assert_eq!(log.lock().unwrap().len(), 3);
    }
}
