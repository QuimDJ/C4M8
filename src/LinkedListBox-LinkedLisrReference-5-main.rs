use std::io::Empty;

#[warn(unused)]
#[derive(Debug, Clone)]
enum LinkedListUsingBox<T> {
    Empty,
    Node {
        value: T,
        next: Box<LinkedListUsingBox<T>>,
    },
}
#[derive(Debug, Clone)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingReference<'a, T>,
    },
}

fn create_list() -> LinkedListUsingBox<i32> {
    let second_node = LinkedListUsingBox::Node {
        value: 2,
        next: Box::new(LinkedListUsingBox::Empty),
    };

    let first_node = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(second_node),
    };

    first_node
}
fn main() {
    let list = create_list();
    println!("{list:#?}");
}
