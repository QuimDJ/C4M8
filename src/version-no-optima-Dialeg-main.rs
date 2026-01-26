#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node { valor: T, next: Box<LinkedList<T>> },
}
impl<T> LinkedList<T> {
    fn new() -> Box<LinkedList<T>> {
        Box::new(LinkedList::Empty)
    }
    fn add_node(self, target: T) -> LinkedList<T> {
        match self {
            LinkedList::Empty => LinkedList::Node {
                valor: target,
                next: Box::new(LinkedList::Empty),
            },
            LinkedList::Node { valor, next } => LinkedList::Node {
                valor: valor,
                next: Box::new(next.add_node(target)),
            },
        }
    }
}
fn main() {
    let mut t: Box<LinkedList<i32>> = LinkedList::new();
    //println!("{:#?}", t);
    let t = t.add_node(7);
    println!("{:#?}", t);
    let t = t.add_node(10);
    println!("{:#?}", t);
}
