#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node { valor: T, next: Box<LinkedList<T>> },
}
impl<T> LinkedList<T> {
    fn new() -> Box<LinkedList<T>> {
        Box::new(LinkedList::Empty)
    }
    fn add_node(&mut self, target: T) {
        let mut current: &mut LinkedList<T> = self;

        loop {
            match current {
                LinkedList::Empty => {
                    *current = LinkedList::Node {
                        valor: target,
                        next: Box::new(LinkedList::Empty),
                    };
                    break;
                }
                LinkedList::Node { next, .. } => {
                    current = &mut *next; // 👈 aquí está la clave
                }
            }
        }
    }
}
fn main() {
    let mut t: Box<LinkedList<i32>> = LinkedList::new();
    //println!("{:#?}", t);
    t.add_node(7);
    println!("{:#?}", t);
    t.add_node(5);
    println!("{:#?}", t);
}
