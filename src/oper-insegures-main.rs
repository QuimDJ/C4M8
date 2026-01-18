fn main() {
    // No sujeto a las reglas del borrow checker de Rust compiler
    let mut x = 5;
    let p1 = &mut x as *mut i32;
    let p2 = &mut x as *mut i32; // permitido

    println!("{:?}", p2);
    unsafe {
        *p2 = *p1 + 4;
        println!("{:?}", *p1);
    }
}
