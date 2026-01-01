use std::rc::Rc;

/// A tag that can be shared across multiple items.
/// Example: "Rust", "Tutorial", "Beginner"
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

/// An item that holds a shared reference to a Tag.
/// Multiple Items can share the same Tag via Rc.
#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub tag: Rc<Tag>,
}

impl Item {
    pub fn new(name: &str, tag: Rc<Tag>) -> Self {
        Item {
            name: name.to_string(),
            tag,
        }
    }
}

pub fn demo_shared_tag() {
    let tag = Rc::new(Tag::new("Rust"));
    println!("{}", Rc::strong_count(&tag));

    let item1 = Item::new("blog", Rc::clone(&tag));
    println!("{}", Rc::strong_count(&tag));

    let item2 = Item::new("tutorial", Rc::clone(&tag));
    println!("{}", Rc::strong_count(&tag));

    let item3 = Item::new("video", Rc::clone(&tag));
    println!("{}", Rc::strong_count(&tag));

    println!("\n\n");
    println!("  item1.tag.name = {}", item1.tag.name);
    println!("  item2.tag.name = {}", item2.tag.name);
    println!("  item3.tag.name = {}", item3.tag.name);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_demo_shared_tag() {
        demo_shared_tag();
    }

    #[test]
    fn test_strong_count_equals_shared_items() {
        let tag = Rc::new(Tag::new("Concurrency"));
        assert_eq!(Rc::strong_count(&tag), 1);

        let items: Vec<Item> = (1..=5)
            .map(|idx| Item::new(&format!("Item {}", idx), Rc::clone(&tag)))
            .collect();

        assert_eq!(Rc::strong_count(&tag), 6);

        // Consumes `items`
        for item in items.into_iter() {
            assert_eq!(item.tag.name, "Concurrency")
        }
    }

    // NOTE: This test intentionally does NOT compile!
    // It demonstrates that Rc is not thread-safe (not Send).
    // Uncomment to see the compiler error.
    //
    // use std::thread;
    // #[test]
    // fn test_rc_is_not_send() {
    //     use std::thread;
    //     let tag = Rc::new(Tag::new("ThreadTest"));
    //     thread::spawn(move || println!("{}", tag.name));
    // }
}
