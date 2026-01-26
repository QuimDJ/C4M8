use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct CustomBox<T, U> {
    data: T,
    more_data: U,
}
impl<T, U> CustomBox<T, U> {
    fn new(data: T, more_data: U) -> Self {
        Self { data, more_data }
    }
}
impl<T, U> Deref for CustomBox<T, U> {
    type Target = U;
    fn deref(&self) -> &Self::Target {
        &self.more_data
    }
}
impl<T, U> DerefMut for CustomBox<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more_data
    }
}
fn main() {
    let mut boxy = Box::new(3.14);
    println!("{}", *boxy);
    *boxy = 5.0;

    let mut custom_boxy = CustomBox {
        data: 3.14,
        more_data: String::from("felicidad"),
    };
    *custom_boxy = String::from("updated");
    println!("{}", *custom_boxy);
}
