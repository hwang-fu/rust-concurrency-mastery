use std::sync::{Arc, RwLock};

/// Handler type: a thread-safe function that receives event references.
type Handler<E> = Arc<dyn Fn(&E) + Send + Sync>;

/// A thread-safe pub-sub event bus.
///
/// - Subscribe handlers with `subscribe()`
/// - Publish events with `publish()` — all handlers are called
pub struct EventBus<E> {
    handlers: Arc<RwLock<Vec<Handler<E>>>>,
}

impl<E> EventBus<E> {
    pub fn new() -> Self {
        let handlers = Arc::new(RwLock::new(Vec::new()));
        EventBus { handlers }
    }

    pub fn subscribe<F>(&self, handler: F)
    where
        F: Fn(&E) + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers.push(Arc::new(handler));
    }

    pub fn publish(&self, event: &E) {
        let handlers = self.handlers.read().unwrap();
        for handler in handlers.iter() {
            handler(event);
        }
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.read().unwrap().len()
    }
}

impl<E> Default for EventBus<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Clone for EventBus<E> {
    fn clone(&self) -> Self {
        let handlers = Arc::clone(&self.handlers);
        EventBus { handlers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    #[test]
    fn test_basic_subscribe_publish() {
        let bus = EventBus::<String>::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        bus.subscribe(move |_event: &String| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        bus.publish(&"hello".to_string());
        bus.publish(&"world".to_string());

        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_multiple_handlers() {
        let bus = EventBus::<i32>::new();
        let counter = Arc::new(AtomicUsize::new(0));

        // Subscribe 3 handlers
        for _ in 0..3 {
            let counter_clone = Arc::clone(&counter);
            bus.subscribe(move |_event: &i32| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        assert_eq!(bus.handler_count(), 3);

        // Publish 5 events
        for i in 0..5 {
            bus.publish(&i);
        }

        // 3 handlers × 5 events = 15 calls
        assert_eq!(counter.load(Ordering::SeqCst), 15);
    }

    #[test]
    fn test_concurrent_publish() {
        let bus = EventBus::<i32>::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = Arc::clone(&counter);
        bus.subscribe(move |event: &i32| {
            counter_clone.fetch_add(*event as usize, Ordering::SeqCst);
        });

        let mut handles = vec![];

        // 4 threads, each publishing 10 events
        for _ in 0..4 {
            let bus_clone = bus.clone();
            handles.push(thread::spawn(move || {
                for i in 0..10 {
                    bus_clone.publish(&i);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Each thread publishes 0+1+2+...+9 = 45
        // 4 threads × 45 = 180
        assert_eq!(counter.load(Ordering::SeqCst), 180);
    }

    #[test]
    fn test_handler_receives_event_data() {
        let bus = EventBus::<String>::new();
        let received = Arc::new(RwLock::new(Vec::new()));

        let received_clone = Arc::clone(&received);
        bus.subscribe(move |event: &String| {
            received_clone.write().unwrap().push(event.clone());
        });

        bus.publish(&"first".to_string());
        bus.publish(&"second".to_string());

        let messages = received.read().unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], "first");
        assert_eq!(messages[1], "second");
    }
}
