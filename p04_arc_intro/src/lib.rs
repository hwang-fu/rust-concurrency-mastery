use std::{sync::Arc, thread};

#[derive(Debug)]
pub struct Tag {
    pub name: String,
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Tag {
            name: name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub tag: Arc<Tag>,
}

impl Item {
    pub fn new(name: &str, tag: Arc<Tag>) -> Self {
        Item {
            name: name.to_string(),
            tag,
        }
    }
}

pub fn demo_arc_threads() {
    let tag = Arc::new(Tag::new("Rust"));
    println!("{}", Arc::strong_count(&tag));

    let mut handles = vec![];
    for i in 1..=3 {
        let cloned_tag = Arc::clone(&tag);
        let handle = thread::spawn(move || {
            println!(
                "Thread {}: tag = {:?}, strong_count = {}",
                i,
                cloned_tag,
                Arc::strong_count(&cloned_tag)
            );
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked")
    }

    println!("{}", Arc::strong_count(&tag));
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn test_demo_arc_threads() {
        demo_arc_threads();
    }

    #[test]
    fn test_arc_is_send_and_sync() {
        let tag = Arc::new(Tag::new("thread safe tag"));
        assert_eq!(Arc::strong_count(&tag), 1);

        let cloned_tag = Arc::clone(&tag);
        thread::spawn(move || {
            assert_eq!(cloned_tag.name, "thread safe tag");
        })
        .join()
        .expect("Thread panicked");

        assert_eq!(Arc::strong_count(&tag), 1);
    }

    #[test]
    fn test_multiple_threads_share_same_data() {
        let tag = Arc::new(Tag::new("shared"));
        let cnt = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];

        for _ in 1..=10 {
            let cloned_tag = Arc::clone(&tag);
            let cloned_cnt = Arc::clone(&cnt);

            handles.push(thread::spawn(move || {
                assert_eq!(cloned_tag.name, "shared");
                cloned_cnt.fetch_add(1, Ordering::SeqCst);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(cnt.load(Ordering::SeqCst), 10);
    }
}
