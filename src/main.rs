#[derive(Debug)]
enum BinarySearchTree {
    Empty,
    Node {
        value: i32,
        left: Box<BinarySearchTree>,
        right: Box<BinarySearchTree>,
    },
}

fn main() {
    let tree = BinarySearchTree::Node {
        value: 8,
        left: Box::new(BinarySearchTree::Node {
            value: 3,
            left: Box::new(BinarySearchTree::Node {
                value: 1,
                left: Box::new(BinarySearchTree::Empty),
                right: Box::new(BinarySearchTree::Empty),
            }),
            right: Box::new(BinarySearchTree::Node {
                value: 6,
                left: Box::new(BinarySearchTree::Empty),
                right: Box::new(BinarySearchTree::Node {
                    value: 7,
                    left: Box::new(BinarySearchTree::Empty),
                    right: Box::new(BinarySearchTree::Empty),
                }),
            }),
        }),
        right: Box::new(BinarySearchTree::Node {
            value: 10,
            left: Box::new(BinarySearchTree::Empty),
            right: Box::new(BinarySearchTree::Node {
                value: 14,
                left: Box::new(BinarySearchTree::Empty),
                right: Box::new(BinarySearchTree::Empty),
            }),
        }),
    };

    println!("{:#?}", tree);
}
