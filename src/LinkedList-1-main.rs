#[derive(Debug, Clone)]
enum LinkedList {
    Empty,
    Node { value: i32, next: Box<LinkedList> },
}

fn main() {
    let list = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Node {
            value: 2,
            next: Box::new(LinkedList::Node {
                value: 3,
                next: Box::new(LinkedList::Node {
                    value: 4,
                    next: Box::new(LinkedList::Empty),
                }),
            }),
        }),
    };

    let list = LinkedList::Node {
        value: 0,
        next: Box::new(list),
    };

    println!("{list:#?}");
}
