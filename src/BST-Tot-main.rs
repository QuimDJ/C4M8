use std::cmp::Ordering;

#[derive(Debug)]
enum BinarySearchTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinarySearchTree<T>>,
        right: Box<BinarySearchTree<T>>,
    },
}

impl<T: Ord> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree::Empty
    }

    fn insert(self, value: T) -> BinarySearchTree<T> {
        match self {
            BinarySearchTree::Empty => BinarySearchTree::Node {
                value,
                left: Box::new(BinarySearchTree::Empty),
                right: Box::new(BinarySearchTree::Empty),
            },

            BinarySearchTree::Node {
                value: v,
                left,
                right,
            } => match value.cmp(&v) {
                Ordering::Less => BinarySearchTree::Node {
                    value: v,
                    left: Box::new(left.insert(value)),
                    right,
                },

                Ordering::Greater => BinarySearchTree::Node {
                    value: v,
                    left,
                    right: Box::new(right.insert(value)),
                },

                Ordering::Equal => BinarySearchTree::Node {
                    value: v,
                    left,
                    right,
                },
            },
        }
    }

    fn contains(&self, target: &T) -> bool {
        match self {
            BinarySearchTree::Empty => false,

            BinarySearchTree::Node { value, left, right } => match target.cmp(value) {
                Ordering::Equal => true,
                Ordering::Less => left.contains(target),
                Ordering::Greater => right.contains(target),
            },
        }
    }

    fn remove(self, target: &T) -> BinarySearchTree<T> {
        match self {
            BinarySearchTree::Empty => BinarySearchTree::Empty,

            BinarySearchTree::Node { value, left, right } => {
                match target.cmp(&value) {
                    Ordering::Less => BinarySearchTree::Node {
                        value,
                        left: Box::new(left.remove(target)),
                        right,
                    },

                    Ordering::Greater => BinarySearchTree::Node {
                        value,
                        left,
                        right: Box::new(right.remove(target)),
                    },

                    Ordering::Equal => match (*left, *right) {
                        // sin hijos
                        (BinarySearchTree::Empty, BinarySearchTree::Empty) => {
                            BinarySearchTree::Empty
                        }

                        // un hijo
                        (left, BinarySearchTree::Empty) => left,
                        (BinarySearchTree::Empty, right) => right,

                        // dos hijos
                        (left, right) => {
                            let (min, new_right) = right.extract_min();
                            BinarySearchTree::Node {
                                value: min,
                                left: Box::new(left),
                                right: Box::new(new_right),
                            }
                        }
                    },
                }
            }
        }
    }

    fn extract_min(self) -> (T, BinarySearchTree<T>) {
        match self {
            BinarySearchTree::Node { value, left, right } => match *left {
                BinarySearchTree::Empty => (value, *right),
                left => {
                    let (min, new_left) = left.extract_min();
                    (
                        min,
                        BinarySearchTree::Node {
                            value,
                            left: Box::new(new_left),
                            right,
                        },
                    )
                }
            },

            BinarySearchTree::Empty => {
                panic!("extract_min llamado sobre un árbol vacío");
            }
        }
    }
}

fn main() {
    // Crear árbol
    let tree = BinarySearchTree::new()
        .insert(8)
        .insert(3)
        .insert(10)
        .insert(1)
        .insert(6)
        .insert(14)
        .insert(4)
        .insert(7)
        .insert(13);

    println!("Árbol inicial:\n{:#?}", tree);

    // contains
    println!("Contiene 6? {}", tree.contains(&6));
    println!("Contiene 2? {}", tree.contains(&2));

    // remove hoja
    let tree = tree.remove(&1);
    println!("\nDespués de eliminar 1:\n{:#?}", tree);

    // remove nodo con un hijo
    let tree = tree.remove(&14);
    println!("\nDespués de eliminar 14:\n{:#?}", tree);

    // remove nodo con dos hijos
    let tree = tree.remove(&3);
    println!("\nDespués de eliminar 3:\n{:#?}", tree);

    // remove raíz
    let tree = tree.remove(&8);
    println!("\nDespués de eliminar la raíz (8):\n{:#?}", tree);
}
