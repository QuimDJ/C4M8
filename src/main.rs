#[derive(Debug, Clone)]
enum LinkedList {
    Empty,
    Node { value: i32, next: Box<LinkedList> },
}
impl LinkedList {
    fn get(&self, index: usize) -> Option<i32> {
        match (self, index) {
            (LinkedList::Empty, _) => None,
            (LinkedList::Node { value, .. }, 0) => Some(*value),
            (LinkedList::Node { next, .. }, i) => next.get(i - 1),
        }
    }
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

    println!("{:#?}", list);
    if let Some(value) = list.get(2) {
        println!("Tercer elemento: {}", value);
    }
}
