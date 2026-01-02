# Rust Concurrency Primitives Mastery

A progressive, hands-on micro-project series for mastering `Rc`, `Arc`, `Mutex`, `RwLock`, and `Weak` through real coding.

## Overview

This workspace contains 11 micro-projects that build upon each other, taking you from basic reference counting to advanced patterns like thread pools and event buses.

## Project Structure

```
rust-concurrency-mastery/
├── p01_rc_basics/          # Rc basics with tag system
├── p02_rc_tree/            # Tree structures with Rc
├── p03_weak_doubly_linked/ # Doubly-linked list with Weak
├── p04_arc_intro/          # Arc across threads
├── p05_arc_counter/        # Atomic vs Mutex benchmark
├── p06_mutex_basics/       # Thread-safe log collector
├── p07_rwlock_cache/       # Read-optimized cache
├── p08_deadlock_lab/       # Deadlock scenarios and fixes
├── p09_threadpool/         # Thread pool with job queue
├── p10_event_bus/          # Pub-sub event system
└── p11_pitfalls/           # Common mistakes & debugging
```

## Concepts Covered

### Phase 1: `Rc` & `Weak` — Single-Threaded Shared Ownership

| Project | Concept | Key Takeaway |
|---------|---------|--------------|
| `p01_rc_basics` | Reference counting | `Rc::clone()` is cheap, increments count |
| `p02_rc_tree` | Tree traversal | `&Rc<T>` for reading, clone for storing |
| `p03_weak_doubly_linked` | Cycle prevention | `Weak` doesn't prevent deallocation |

### Phase 2: `Arc` — Thread-Safe Shared Ownership

| Project | Concept | Key Takeaway |
|---------|---------|--------------|
| `p04_arc_intro` | Multi-threaded sharing | `Arc` = atomic `Rc`, is `Send + Sync` |
| `p05_arc_counter` | Interior mutability | `Arc` alone is read-only; use `Atomic*` or `Mutex` |

### Phase 3: `Mutex` & `RwLock` — Interior Mutability

| Project | Concept | Key Takeaway |
|---------|---------|--------------|
| `p06_mutex_basics` | Exclusive locking | Guard scope = lock duration |
| `p07_rwlock_cache` | Read-write locking | `RwLock` ~2x faster for read-heavy workloads |
| `p08_deadlock_lab` | Deadlock patterns | Consistent lock ordering prevents deadlocks |

### Phase 4: Real-World Patterns

| Project | Concept | Key Takeaway |
|---------|---------|--------------|
| `p09_threadpool` | Job queue | `Arc<Mutex<Receiver>>` for shared channel |
| `p10_event_bus` | Pub-sub pattern | `Arc<RwLock<Vec<Handler>>>` for dynamic handlers |

### Phase 5: Pitfalls & Debugging

| Module | Pitfall | Fix |
|--------|---------|-----|
| `clone_confusion` | Moving Arc instead of cloning | Always `Arc::clone()` before `move` |
| `poisoned_mutex` | Panic while holding lock | `into_inner()` or fail fast |
| `arc_cycle_leak` | Reference cycles | Use `Weak` for back-references |
| `unnecessary_arc` | Overusing Arc | Prefer move, scoped threads, or channels |
| `diagnostics` | Debugging techniques | `strong_count()`, `Drop` tracing |

## Quick Reference

| Primitive | Thread-Safe | Use Case |
|-----------|-------------|----------|
| `Rc<T>` | No | Single-threaded shared ownership |
| `Weak<T>` | No | Break cycles with `Rc` |
| `Arc<T>` | Yes | Multi-threaded shared ownership |
| `Mutex<T>` | Yes | Exclusive mutable access |
| `RwLock<T>` | Yes | Many readers OR one writer |
| `RefCell<T>` | No | Interior mutability (single-threaded) |
| `Atomic*` | Yes | Lock-free primitives for simple types |

## Running the Projects

```bash
# Run all tests
cargo test

# Run tests for a specific project
cargo test -p p09_threadpool

# Run with output visible
cargo test -p p07_rwlock_cache -- --nocapture

# Check a specific project compiles
cargo check -p p11_pitfalls
```

## Key Patterns Learned

### 1. Arc + Mutex for Shared Mutable State
```rust
let data = Arc::new(Mutex::new(Vec::new()));
let data_clone = Arc::clone(&data);
thread::spawn(move || {
    data_clone.lock().unwrap().push(42);
});
```

### 2. Arc + RwLock for Read-Heavy Workloads
```rust
let cache = Arc::new(RwLock::new(HashMap::new()));
// Multiple readers simultaneously
let value = cache.read().unwrap().get(&key);
// Exclusive writer
cache.write().unwrap().insert(key, value);
```

### 3. Graceful Shutdown via Channel Close
```rust
// Drop sender to signal shutdown
drop(sender);
// Receivers get Err, exit their loops
while let Ok(job) = receiver.recv() { ... }
```

### 4. Weak References for Back-Pointers
```rust
struct Node {
    parent: Option<Weak<Node>>,  // Won't prevent parent from dropping
    children: Vec<Rc<Node>>,     // Strong ownership of children
}
```

## Requirements

- Rust 1.70+ (uses scoped threads, let chains)
- No external dependencies (std only)

## License

MIT
