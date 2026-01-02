use std::{
    collections::HashMap,
    hash::Hash,
    sync::{
        Arc, Mutex, RwLock,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Instant,
};

/// A simple key-value cache.
/// Thread-safety will come from wrapping this in Arc<RwLock<Cache>>.
pub struct Cache<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.data.get(key).cloned()
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<K, V> Default for Cache<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

pub fn demo_rwlock_cache() {
    let cache = Arc::new(RwLock::new(Cache::<String, i32>::new()));

    let mut handles = vec![];

    for reader_id in 1..=8 {
        let cloned_cache = Arc::clone(&cache);

        handles.push(thread::spawn(move || {
            for i in 1..=1000 {
                let guard = cloned_cache.read().unwrap();
                let _ = guard.get(&format!("key_{}", i % 100));
                // read lock released here
            }
            println!("Reader {} finished", reader_id)
        }));
    }

    for writer_id in 1..=2 {
        let cloned_cache = Arc::clone(&cache);

        handles.push(thread::spawn(move || {
            for i in 1..=100 {
                let mut guard = cloned_cache.write().unwrap();
                guard.insert(format!("key_{}", i), writer_id * 1000 + i);
                // write lock released here
            }
            println!("Writer {} finished", writer_id);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_cache = cache.read().unwrap();
    println!("Final cache size: {} entries", final_cache.len());
}

/// Benchmark using RwLock - readers can run in parallel
pub fn benchmark_rwlock(num_readers: usize, num_writers: usize, ops_per_thread: usize) -> u128 {
    let cache = Arc::new(RwLock::new(Cache::<i32, i32>::new()));

    {
        let mut guard = cache.write().unwrap();
        for i in 0..100 {
            guard.insert(i, i * 10);
        }
    }

    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..num_readers {
        let cloned_cache = Arc::clone(&cache);

        handles.push(thread::spawn(move || {
            for i in 0..ops_per_thread {
                let guard = cloned_cache.read().unwrap();
                let _ = guard.get(&((i % 100) as i32));
            }
        }));
    }

    for _ in 0..num_writers {
        let cloned_cache = Arc::clone(&cache);

        handles.push(thread::spawn(move || {
            for i in 0..ops_per_thread {
                let mut guard = cloned_cache.write().unwrap();
                guard.insert((i % 100) as i32, i as i32);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    start.elapsed().as_millis()
}

/// Benchmark using Mutex - all access is exclusive
pub fn benchmark_mutex(num_readers: usize, num_writers: usize, ops_per_thread: usize) -> u128 {
    let cache = Arc::new(Mutex::new(Cache::<i32, i32>::new()));

    // Pre-populate cache
    {
        let mut guard = cache.lock().unwrap();
        for i in 0..100 {
            guard.insert(i, i * 10);
        }
    }

    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..num_readers {
        let cloned_cache = Arc::clone(&cache);

        handles.push(thread::spawn(move || {
            for i in 0..ops_per_thread {
                let guard = cloned_cache.lock().unwrap();
                let _ = guard.get(&((i % 100) as i32));
            }
        }));
    }

    for _ in 0..num_writers {
        let cloned_cache = Arc::clone(&cache);

        handles.push(thread::spawn(move || {
            for i in 0..ops_per_thread {
                let mut guard = cloned_cache.lock().unwrap();
                guard.insert((i % 100) as i32, i as i32);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap()
    }

    start.elapsed().as_millis()
}

pub struct TrackedCache<K, V> {
    inner: RwLock<Cache<K, V>>,
    read_count: AtomicUsize,
    write_count: AtomicUsize,
}

impl<K, V> TrackedCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        TrackedCache {
            inner: RwLock::new(Cache::new()),
            read_count: AtomicUsize::new(0),
            write_count: AtomicUsize::new(0),
        }
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.read_count.fetch_add(1, Ordering::Relaxed);
        let guard = self.inner.read().unwrap();
        guard.get(key)
    }

    pub fn insert(&self, key: K, value: V) {
        self.write_count.fetch_add(1, Ordering::Relaxed);
        let mut guard = self.inner.write().unwrap();
        guard.insert(key, value)
    }

    pub fn stats(&self) -> (usize, usize) {
        (
            self.read_count.load(Ordering::Relaxed),
            self.write_count.load(Ordering::Relaxed),
        )
    }
}

impl<K, V> Default for TrackedCache<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic_operations() {
        let mut cache = Cache::new();

        cache.insert(String::from("key1"), 100);
        cache.insert(String::from("key2"), 200);

        assert_eq!(cache.get(&String::from("key1")), Some(100));
        assert_eq!(cache.get(&String::from("key2")), Some(200));
        assert_eq!(cache.get(&String::from("key3")), None);

        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_demo_rwlock_cache() {
        demo_rwlock_cache();
    }

    #[test]
    fn test_rwlock_cache_concurrent_access() {
        let cache = Arc::new(RwLock::new(Cache::<i32, i32>::new()));
        let mut handles = vec![];

        for writer_id in 0..5 {
            let cloned_cache = Arc::clone(&cache);
            handles.push(thread::spawn(move || {
                for i in 0..10 {
                    let key = writer_id * 10 + i;
                    let mut guard = cloned_cache.write().unwrap();
                    guard.insert(key, key * 100);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        let final_cache = cache.read().unwrap();
        assert_eq!(final_cache.len(), 50); // 5 writers × 10 keys each
    }

    #[test]
    fn benchmark_comparison() {
        let num_readers = 8;
        let num_writers = 2;
        let ops_per_thread = 10_000;

        let rwlock_time = benchmark_rwlock(num_readers, num_writers, ops_per_thread);
        let mutex_time = benchmark_mutex(num_readers, num_writers, ops_per_thread);

        println!("\n=== Benchmark Results ===");
        println!(
            "Readers: {}, Writers: {}, Ops/thread: {}",
            num_readers, num_writers, ops_per_thread
        );
        println!("RwLock: {}ms", rwlock_time);
        println!("Mutex:  {}ms", mutex_time);

        if mutex_time > 0 && rwlock_time > 0 {
            println!(
                "RwLock is {:.2}x faster",
                mutex_time as f64 / rwlock_time as f64
            );
        }
    }

    #[test]
    fn test_tracked_cache_stats() {
        let cache = Arc::new(TrackedCache::<i32, i32>::new());
        let mut handles = vec![];

        // 4 readers, 100 reads each
        for _ in 0..4 {
            let cloned_cache = Arc::clone(&cache);
            handles.push(thread::spawn(move || {
                for i in 0..100 {
                    let _ = cloned_cache.get(&i);
                }
            }));
        }

        // 2 writers, 50 writes each
        for _ in 0..2 {
            let cloned_cache = Arc::clone(&cache);
            handles.push(thread::spawn(move || {
                for i in 0..50 {
                    cloned_cache.insert(i, i * 10);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        let (reads, writes) = cache.stats();
        assert_eq!(reads, 400); // 4 readers × 100 reads
        assert_eq!(writes, 100); // 2 writers × 50 writes
        println!("TrackedCache stats: {} reads, {} writes", reads, writes);
    }
}
