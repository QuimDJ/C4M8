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

fn main() {
    /*
    // References Sample
    let second_node = LinkedListUsingReference::Node {
        value: 1,
        next: &LinkedListUsingReference::Node {
            value: 2,
            next: &LinkedListUsingReference::Empty,
        },
    };
    let first_node = LinkedListUsingReference::Node {
        value: 0,
        next: &second_node,
    };

    // if second_node is deallocated, we have some problems about dangling reference in first_node.
    // if first_node is deallocated, second_node can continue existing by itself.
    println!("{:#?}", second_node);*/

    // Box<LinledList<Box<T>> Sample

    let second_node = LinkedListUsingBox::Node {
        value: 2,
        next: Box::new(LinkedListUsingBox::Empty),
    };
    //println!("{:#?}", second_node);

    let first_node = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(second_node),
    };
    drop(first_node);
    // In this example Box<T> of first node gets owenrship and alert about
    // impossible access to second_node, because Box<T> had got ownership of second_node.
    // So Box<T> is more secure than References
    println!("{:#?}", second_node);
    // The interesting part of this second sample is that linkedList with Box, have Box as a guardian of
    // the links. We cannot delete or drop one part without the alert of Box, the owner.
}
