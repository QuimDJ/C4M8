use std::error::Error;
use std::fs;

fn read_number_from_file(path: &str) -> Result<i32, Box<dyn Error>> {
    let file_contents = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => return Err(Box::new(error)),
    };
    match file_contents.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(error) => Err(Box::new(error)),
    }
}

fn main() {
    match read_number_from_file("value.txt") {
        Ok(value) => println!("The number is {value}"),
        Err(error) => println!("The error is {error}"),
    }
}
