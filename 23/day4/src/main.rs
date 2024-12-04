use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn calculate_card_score(line: &str) -> usize {
    let numbers = line.split(":").collect::<Vec<&str>>()[1];
    let split_numbers = numbers.split("|").collect::<Vec<&str>>();
    let winning_numbers_str = split_numbers[0];
    let my_numbers_str = split_numbers[1];

    let mut winning_set = HashSet::new();
    winning_numbers_str.split_whitespace().for_each(|num| {
        winning_set.insert(num);
    });

    let mut score = 0;
    my_numbers_str.split_whitespace().for_each(|num| {
        if winning_set.contains(num) {
            score += 1;
        }
    });

    score
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let mut scratchcard_count = 0;

    let mut copies: HashMap<usize, usize> = HashMap::new();

    contents.lines().enumerate().for_each(|(idx, line)| {
        let all_copies_count = if let Some(count) = copies.get(&idx) {
            count + 1
        } else {
            1
        };
        scratchcard_count += all_copies_count;

        println!("Handled {} for card {}", all_copies_count, idx);

        let score = calculate_card_score(line);

        for i in idx + 1..idx + score + 1 {
            let new_count = if let Some(count) = copies.get(&i) {
                count + all_copies_count
            } else {
                all_copies_count
            };

            println!("Added {} copies to line {}", 1, i);

            copies.insert(i, new_count);
        }
    });

    println!("{}", scratchcard_count);
}
