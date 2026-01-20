#[derive(Debug)]
enum LinkedListUsingBox<T> {
    Empty,
    node {
        value: T,
        next: Box<LinkedListUsingBox<T>>,
    },
}

enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingReference<'a, T>,
    },
}

fn main() {}
