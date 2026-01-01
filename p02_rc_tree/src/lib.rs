use std::rc::Rc;

/// A tree node that holds a value and references to its children.
/// Children are shared via Rc, allowing multiple references to the same node.
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Rc<Node<T>>>,
    pub right: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn with_children(value: T, left: Rc<Node<T>>, right: Rc<Node<T>>) -> Self {
        Node {
            value,
            left: Some(left),
            right: Some(right),
        }
    }

    pub fn with_left_child(value: T, left: Rc<Node<T>>) -> Self {
        Node {
            value,
            left: Some(left),
            right: None,
        }
    }

    pub fn with_right_child(value: T, right: Rc<Node<T>>) -> Self {
        Node {
            value,
            left: None,
            right: Some(right),
        }
    }
}

pub fn count_nodes<T>(root: &Rc<Node<T>>) -> usize {
    let left_children_count = match root.left.as_ref() {
        Some(left) => count_nodes(left),
        None => 0,
    };

    let right_children_count = match root.right.as_ref() {
        Some(right) => count_nodes(right),
        None => 0,
    };

    1 + left_children_count + right_children_count
}

pub fn find_node<T: PartialEq>(root: &Rc<Node<T>>, value: &T) -> Option<Rc<Node<T>>> {
    if &root.value == value {
        return Some(Rc::clone(root));
    }

    if let Some(left) = root.left.as_ref()
        && let Some(found) = find_node(left, value)
    {
        return Some(found);
    }

    if let Some(right) = root.right.as_ref()
        && let Some(found) = find_node(right, value)
    {
        return Some(found);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds this tree:
    ///        A
    ///       / \
    ///      B   C
    ///     /     \
    ///    D       E
    fn build_test_tree() -> Rc<Node<&'static str>> {
        let d = Rc::new(Node::new("D"));
        let e = Rc::new(Node::new("E"));

        let b = Rc::new(Node::with_left_child("B", Rc::clone(&d)));
        let c = Rc::new(Node::with_right_child("C", Rc::clone(&e)));

        Rc::new(Node::with_children("A", Rc::clone(&b), Rc::clone(&c)))
    }

    #[test]
    fn test_count_nodes() {
        let root = build_test_tree();
        assert_eq!(count_nodes(&root), 5);
    }

    #[test]
    fn test_find_node_exists() {
        let root = build_test_tree();

        let found = find_node(&root, &"C");

        assert!(found.is_some());
        assert_eq!(found.unwrap().value, "C");
    }

    #[test]
    fn test_find_node_not_exists() {
        let root = build_test_tree();

        let found = find_node(&root, &"Z");
        assert!(found.is_none());
    }

    #[test]
    fn test_parent_pointer_cycle_problem() {
        // This test demonstrates WHY we can't use Rc for parent pointers.
        // We'll just show the concept with strong_count.

        let root = build_test_tree();

        // Root has 1 reference (our variable)
        println!("root strong_count: {}", Rc::strong_count(&root));

        // If child B had an Rc back to root, root would have 2+ references.
        // When we drop `root`, the count would go to 1 (child's reference).
        // Child can't be dropped because root still references it.
        // Root can't be dropped because child still references it.
        // = CYCLE = MEMORY LEAK

        // The solution? Weak<T> (Phase 1.3)
        // Weak doesn't increment strong_count, so cycles can be broken.
    }
}
