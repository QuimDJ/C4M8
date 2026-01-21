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

fn create_list<'a>() -> LinkedListUsingReference<'a, i32> {
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
    // Esta es otra problematica de las referencias
    // Tenemos una referencia a una variable local que desaparecerá, i el compilador bloquea la acción,
    // por posible referencia vacia (dangling reference). La referencia apunta a second_node pero no dispone de su
    // propiedad, la propiedad es de second_node todavia. SE incumplen todas las reglas del retorno de una
    // función: solo un parametro o un valor con propiedad, nunca una referencia (porque al salir de la función se convierte
    // en dangling reference)
    first_node
    // En el ejemplo 5 sustituimos estas referencias por Box<T>
}
fn main() {
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
    //println!("{:#?}", second_node);
}
