use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("input.txt").expect("Error reading file");
    println!("Hello, world!");
}
