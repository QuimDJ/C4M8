fn main() {
    let mut sushi = String::from("Yellowtail");

    let sushi_raw_pointer_1 = &raw const sushi;
    let sushi_raw_pointer_2 = &raw const sushi;
    let sushi_mut_2 = &raw mut sushi;

    println!("{:?}", sushi_mut_2);
    println!("{:?}", sushi_raw_pointer_2);
}
