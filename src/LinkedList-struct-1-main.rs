#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>, // None marca el final de la lista
}

impl Node {
    // Método para obtener el valor en un índice
    fn get(&self, index: usize) -> Option<i32> {
        if index == 0 {
            Some(self.value)
        } else {
            match &self.next {
                Some(next_node) => next_node.get(index - 1),
                None => None,
            }
        }
    }
}

fn main() {
    // Construimos la lista: 1 -> 2 -> 3 -> 4 -> None
    let list = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: Some(Box::new(Node {
                value: 3,
                next: Some(Box::new(Node {
                    value: 4,
                    next: None, // Fin de la lista
                })),
            })),
        })),
    };

    // Añadimos un nodo al inicio: 0 -> 1 -> 2 -> 3 -> 4 -> None
    let list = Node {
        value: 0,
        next: Some(Box::new(list)),
    };

    println!("{:#?}", list);

    if let Some(value) = list.get(2) {
        println!("Tercer elemento: {}", value);
    }
}
