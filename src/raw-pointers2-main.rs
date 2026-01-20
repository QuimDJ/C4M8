fn main() {
    let mut sushi = String::from("Yellowtail");
    let sushi_raw_pointer_1 = &raw const sushi;
    // Otra forma alternativa es declarar el mismo tipo y asignarle una referencia regular, porque Rust lo convierte al tipo especificado.
    let sushi_raw_pointer2: *const String = &sushi;
    // Hasta este momento solo hemos declarado Raw Pointers que actuan como Referencias regulares IMMUTABLES.
    //println!("{:?}", sushi_raw_pointer_1);
    // AHORA, pasamos a MUTABLES
    let sushi_mutable_raw_pointer_1 = &raw mut sushi;
    // Hemos podido declarar un puntero immutable y uno mutable a la misma variable, eso no era posible en referencias regulares.
    // Incluso podemos declarar un segundo puntero mutable. Aunque no podrán coexistir, el compilador no nos lo permitirá.
    let sushi_mutable_raw_pointer_2 = &raw mut sushi;
    // Cuando tendremos problemas es a la hora de actualizar los valores de esos punteros o al desreferenciar. Para poderlo hacer
    // Hará falta la keyword: 'unsafe'.
    unsafe {
        println!("{}", *sushi_raw_pointer_1);
    }
    // Supongamos que borramos la variable de la memoria
    drop(sushi);
    unsafe {
        println!("{}", *sushi_raw_pointer_1);
    }
}
