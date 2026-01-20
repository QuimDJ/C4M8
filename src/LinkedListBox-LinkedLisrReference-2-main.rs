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

fn main() {
    let list1 = LinkedListUsingReference::Node {
        value: 1,
        next: &LinkedListUsingReference::Node {
            value: 2,
            next: &LinkedListUsingReference::Empty,
        },
    };
    let list1 = LinkedListUsingReference::Node {
        value: 0,
        next: &list1,
    };

    println!("{:#?}", list1);
}
