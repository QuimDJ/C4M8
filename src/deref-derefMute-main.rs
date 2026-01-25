use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomBox<T> {
    data: T,
}
impl<T> CustomBox<T> {
    fn new(data: T) -> Self {
        Self { data }
    }
}
impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> DerefMut for CustomBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
fn main() {
    let mut boxy = Box::new(3.14);
    println!("{}", *boxy);
    *boxy = 5.0;
    let mut custom_boxy = CustomBox::new(3.14);
    println!("{}", *custom_boxy);
    // Next statement don't work because we haven't implemented the DerefMut trait.
    *custom_boxy = 6.6;
    println!("{}", *custom_boxy);

    // Esto funcionaria porque implementa Debug, pero CustomBox<String> no implementa Display
    //let str_CustomBox = CustomBox::new(String::from("TEXT_PROVA"));
    //println!("{:?}", *str_CustomBox);

    // Esto funcionaria porque &String implementa Display
    let str_CustomBox = CustomBox::new(String::from("TEXT_PROVA"));
    println!("{}", *str_CustomBox);
}
