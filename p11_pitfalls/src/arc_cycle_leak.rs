//! Pitfall: Arc Cycle Leak
//!
//! Creating reference cycles with `Arc` causes memory leaks.
//! Use `Weak` for back-references to break cycles.
/// A node that creates a cycle: parent ←→ child both use Rc.
/// This LEAKS memory!
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub mod leaky {
    use super::*;

    pub struct Node {
        pub name: String,
        pub parent: RefCell<Option<Rc<Node>>>,
        pub children: RefCell<Vec<Rc<Node>>>,
    }

    impl Node {
        pub fn new(name: &str) -> Rc<Node> {
            Rc::new(Node {
                name: name.to_string(),
                parent: RefCell::new(None),
                children: RefCell::new(Vec::new()),
            })
        }
    }

    impl Drop for Node {
        fn drop(&mut self) {
            println!("Dropping Node: {}", self.name);
        }
    }

    /// Creates a parent-child cycle. Neither will be dropped!
    pub fn demo_leak() {
        println!("=== Creating Leaky Cycle ===");

        let parent = Node::new("Parent");
        let child = Node::new("Child");

        // Create cycle: parent → child → parent
        parent.children.borrow_mut().push(Rc::clone(&child));
        child.parent.borrow_mut().replace(Rc::clone(&parent));

        println!("parent strong_count: {}", Rc::strong_count(&parent));
        println!("child strong_count: {}", Rc::strong_count(&child));

        println!("Leaving scope...");
        // Drop messages will NOT appear - memory leaked!
    }
}

/// Fixed version: use Weak for parent reference.
pub mod fixed {
    use super::*;

    pub struct Node {
        pub name: String,
        pub parent: RefCell<Option<Weak<Node>>>, // Weak instead of Rc!
        pub children: RefCell<Vec<Rc<Node>>>,
    }

    impl Node {
        pub fn new(name: &str) -> Rc<Node> {
            Rc::new(Node {
                name: name.to_string(),
                parent: RefCell::new(None),
                children: RefCell::new(Vec::new()),
            })
        }
    }

    impl Drop for Node {
        fn drop(&mut self) {
            println!("Dropping Node: {}", self.name);
        }
    }

    /// Creates a parent-child relationship without cycle.
    pub fn demo_no_leak() {
        println!("=== Creating Fixed (No Cycle) ===");

        let parent = Node::new("Parent");
        let child = Node::new("Child");

        // Parent strongly owns child
        parent.children.borrow_mut().push(Rc::clone(&child));
        // Child weakly references parent (doesn't prevent drop)
        child.parent.borrow_mut().replace(Rc::downgrade(&parent));

        println!("parent strong_count: {}", Rc::strong_count(&parent));
        println!("parent weak_count: {}", Rc::weak_count(&parent));
        println!("child strong_count: {}", Rc::strong_count(&child));

        println!("Leaving scope...");
        // Drop messages WILL appear - no leak!
    }
}
