use std::cmp::Ordering;

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> BinaryTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        BinaryTree::Empty
    }
    fn contiene(&self, target: &T) -> bool {
        match self {
            BinaryTree::Empty => false,
            BinaryTree::Node { value, left, right } => match target.cmp(value) {
                Ordering::Equal => true,
                Ordering::Less => left.contiene(target),
                Ordering::Greater => right.contiene(target),
            },
        }
    }
    fn insertar(&mut self, target: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::Node {
                    value: target,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }
            }
            BinaryTree::Node { value, left, right } => match target.cmp(value) {
                Ordering::Equal => {}
                Ordering::Less => left.insertar(target),
                Ordering::Greater => right.insertar(target),
            },
        }
    }
    fn borra(&mut self, target: &T) {
        match self {
            BinaryTree::Empty => {}

            BinaryTree::Node { value, left, right } => match target.cmp(value) {
                Ordering::Less => left.borra(target),
                Ordering::Greater => right.borra(target),

                Ordering::Equal => {
                    match (&**left, &**right) {
                        // 1️⃣ CERO HIJOS
                        (BinaryTree::Empty, BinaryTree::Empty) => {
                            *self = BinaryTree::Empty;
                        }

                        // 2️⃣ SOLO HIJO DERECHO
                        (BinaryTree::Empty, _) => {
                            *self = std::mem::replace(right, BinaryTree::Empty);
                        }

                        // 2️⃣ SOLO HIJO IZQUIERDO
                        (_, BinaryTree::Empty) => {
                            *self = std::mem::replace(left, BinaryTree::Empty);
                        }

                        // 3️⃣ DOS HIJOS
                        (_, _) => {
                            let sucesor = right.extraer_min();
                            *value = sucesor;
                        }
                    }
                }
            },
        }
    }
    fn extraer_min(&mut self) -> T {
        match self {
            BinaryTree::Node { left, right, value } => {
                if let BinaryTree::Empty = **left {
                    let nodo = std::mem::replace(self, BinaryTree::Empty);
                    if let BinaryTree::Node { value, right, .. } = nodo {
                        *self = *right;
                        value
                    } else {
                        unreachable!()
                    }
                } else {
                    left.extraer_min()
                }
            }
            BinaryTree::Empty => unreachable!(),
        }
    }
}

fn main() {
    let mut arbol = BinaryTree::Node {
        value: 8,
        left: Box::new(BinaryTree::Node {
            value: 5,
            left: Box::new(BinaryTree::Node {
                value: 3,
                left: Box::new(BinaryTree::Node {
                    value: 2,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }),
                right: Box::new(BinaryTree::Empty),
            }),
            right: Box::new(BinaryTree::Node {
                value: 6,
                left: Box::new(BinaryTree::Node {
                    value: 4,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }),
                right: Box::new(BinaryTree::Node {
                    value: 7,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }),
            }),
        }),
        right: Box::new(BinaryTree::Node {
            value: 15,
            left: Box::new(BinaryTree::Node {
                value: 13,
                left: Box::new(BinaryTree::Node {
                    value: 11,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }),
                right: Box::new(BinaryTree::Node {
                    value: 14,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }),
            }),
            right: Box::new(BinaryTree::Node {
                value: 18,
                left: Box::new(BinaryTree::Node {
                    value: 17,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }),
                right: Box::new(BinaryTree::Node {
                    value: 20,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }),
            }),
        }),
    };
    println!("{arbol:#?}");
    let v = 6;
    arbol.insertar(13);
    println!("{arbol:#?}");
    println!("Existe el nodo={}? {}", v, arbol.contiene(&v));
    arbol.borra(&8);
    println!("{arbol:#?}");
    //println!("{:?}", arbol::Node{value,BinaryTree::Empty,BinaryTree::Empty);
}
