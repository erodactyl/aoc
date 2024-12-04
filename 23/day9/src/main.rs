use std::fs;

fn is_all_zeroes(numbers: Vec<i64>) -> bool {
    numbers.iter().all(|el| *el == 0)
}

fn get_differences(numbers: Vec<i64>) -> Vec<i64> {
    let mut differences = vec![];
    for i in 1..numbers.len() {
        differences.push(numbers[i] - numbers[i - 1]);
    }
    differences
}

fn get_line_val(line: Vec<i64>) -> i64 {
    let mut difference_mapping = vec![];
    difference_mapping.push(line);

    while !is_all_zeroes(difference_mapping.last().unwrap().to_vec()) {
        difference_mapping.push(get_differences(difference_mapping.last().unwrap().to_vec()));
    }

    difference_mapping.reverse();
    difference_mapping
        .iter()
        .fold(0, |acc, curr| curr.first().unwrap() - acc)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let result: i64 = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|line| get_line_val(line))
        .sum();

    println!("{result}");
}
