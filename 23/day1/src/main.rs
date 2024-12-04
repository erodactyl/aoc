use std::fs;

// https://adventofcode.com/2023/day/1

fn get_calibiration_value(line: &str) -> u32 {
    let mut _line = line;
    let mut digit1: Option<u32> = None;
    let mut digit2: Option<u32> = None;

    let mut set_digits = |digit| {
        if digit1.is_none() {
            digit1 = Some(digit);
        }
        digit2 = Some(digit);
    };

    let mut walk = |line: &str| {
        if line.starts_with("one") || line.starts_with("1") {
            set_digits(1);
        } else if line.starts_with("two") || line.starts_with("2") {
            set_digits(2);
        } else if line.starts_with("three") || line.starts_with("3") {
            set_digits(3);
        } else if line.starts_with("four") || line.starts_with("4") {
            set_digits(4);
        } else if line.starts_with("five") || line.starts_with("5") {
            set_digits(5);
        } else if line.starts_with("six") || line.starts_with("6") {
            set_digits(6);
        } else if line.starts_with("seven") || line.starts_with("7") {
            set_digits(7);
        } else if line.starts_with("eight") || line.starts_with("8") {
            set_digits(8);
        } else if line.starts_with("nine") || line.starts_with("9") {
            set_digits(9);
        }
    };

    while !_line.is_empty() {
        walk(_line);
        let mut chars = _line.chars();
        chars.next();
        _line = chars.as_str();
    }

    println!("{}", line);
    digit1.unwrap() * 10 + digit2.unwrap()
}

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path);

    if contents.is_err() {
        println!("Error: {}", contents.err().unwrap());
        return;
    }

    let sum = contents
        .unwrap()
        .lines()
        .fold(0, |acc, x| acc + get_calibiration_value(x));

    println!("{}", sum);
}
