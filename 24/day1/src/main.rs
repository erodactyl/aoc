use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];

    contents.lines().for_each(|line| {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        first_list.push(numbers[0]);
        second_list.push(numbers[1]);
    });

    // create frequency map for second_list
    // key: number, value: frequency

    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    second_list.iter().for_each(|&x| {
        let count = frequency_map.entry(x).or_insert(0);
        *count += 1;
    });

    let mut similarity_score = 0;

    first_list.iter().for_each(|x| {
        if frequency_map.contains_key(x) {
            similarity_score += x * frequency_map.get(x).unwrap();
        }
    });

    println!("Similarity score: {}", similarity_score);
}
