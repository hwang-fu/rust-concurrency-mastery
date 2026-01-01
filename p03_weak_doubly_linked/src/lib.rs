use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
    pub tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value)));

        match self.tail.take() {
            Some(old_tail_weak) => {
                let old_tail = old_tail_weak.upgrade().expect("tail node should be alive");
                old_tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
            }

            // List is empty — new node is also the head
            None => self.head = Some(Rc::clone(&node)),
        }

        self.tail = Some(Rc::downgrade(&node))
    }

    pub fn traverse_forward(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::new();
        let mut curr = self.head.clone();

        while let Some(node) = curr {
            let node = node.borrow();
            result.push(node.value.clone());
            curr = node.next.clone();
        }

        result
    }

    pub fn traverse_backward(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::new();
        let mut curr = self.tail.clone();

        while let Some(weak) = curr {
            if let Some(node) = weak.upgrade() {
                result.push(node.borrow().value.clone());
                curr = node.borrow().prev.clone();
            } else {
                break;
            }
        }

        result
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn test_push_and_traverse_forward() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.traverse_forward(), vec![1, 2, 3]);
    }

    #[test]
    fn test_traverse_backward() {
        let mut list = LinkedList::new();
        list.push_back("A");
        list.push_back("B");
        list.push_back("C");

        assert_eq!(list.traverse_backward(), vec!["C", "B", "A"]);
    }

    #[test]
    fn test_no_memory_leak() {
        struct DropCounter<'a> {
            counter: &'a Cell<usize>,
        }

        impl<'a> Drop for DropCounter<'a> {
            fn drop(&mut self) {
                self.counter.set(self.counter.get() + 1);
            }
        }

        let drop_count = Cell::new(0);

        {
            let mut list = LinkedList::new();
            list.push_back(DropCounter {
                counter: &drop_count,
            });
            list.push_back(DropCounter {
                counter: &drop_count,
            });
            list.push_back(DropCounter {
                counter: &drop_count,
            });
            assert_eq!(drop_count.get(), 0); // Nothing dropped yet
        }

        assert_eq!(drop_count.get(), 3);
    }
}
