use std::fs;

#[derive(Debug)]
struct SchematicNumber {
    line_index: usize,
    start_index: usize,
    end_index: usize,
    number: usize,
}

fn parse_line(line: &str, line_index: usize) -> Vec<SchematicNumber> {
    let mut numbers: Vec<SchematicNumber> = vec![];
    let mut current_number = String::new();
    let mut index = 0;

    for c in line.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if !current_number.is_empty() {
            let schematic_number = SchematicNumber {
                line_index,
                number: current_number.parse::<usize>().unwrap(),
                end_index: index,
                start_index: index - current_number.len(),
            };
            numbers.push(schematic_number);
            current_number = String::new();
        }
        index += 1;
    }

    if !current_number.is_empty() {
        let schematic_number = SchematicNumber {
            line_index,
            number: current_number.parse::<usize>().unwrap(),
            end_index: index,
            start_index: index - current_number.len(),
        };
        numbers.push(schematic_number);
    }

    numbers
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");

    let grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut asterisk_map = std::collections::HashMap::new();
    let mut sum = 0;

    let mut calculate = |num: &SchematicNumber| {
        let start_index = if num.start_index == 0 {
            0
        } else {
            num.start_index - 1
        };
        let end_index = if num.end_index == grid[num.line_index].len() {
            grid[num.line_index].len()
        } else {
            num.end_index + 1
        };
        // previous line
        if num.line_index != 0 {
            for i in start_index..end_index {
                if grid[num.line_index - 1][i] == '*' {
                    let key = (num.line_index - 1, i);
                    println!("Previous line {:?}, {:?}", num, key);
                    if let Some(pair) = asterisk_map.get(&key) {
                        println!("{} * {:?}", pair, num);
                        sum += pair * num.number;
                    } else {
                        asterisk_map.insert(key, num.number);
                    }
                }
            }
        }
        // current line
        for i in start_index..end_index {
            if grid[num.line_index][i] == '*' {
                let key = (num.line_index, i);
                println!("Current line {:?}, {:?}", num, key);
                if let Some(pair) = asterisk_map.get(&key) {
                    println!("{} * {:?}", pair, num);
                    sum += pair * num.number;
                } else {
                    asterisk_map.insert(key, num.number);
                }
            }
        }
        // next line
        if num.line_index < grid.len() - 1 {
            for i in start_index..end_index {
                if grid[num.line_index + 1][i] == '*' {
                    let key = (num.line_index + 1, i);
                    println!("Next line {:?}, {:?}", num, key);
                    if let Some(pair) = asterisk_map.get(&key) {
                        println!("{} * {:?}", pair, num);
                        sum += pair * num.number;
                    } else {
                        asterisk_map.insert(key, num.number);
                    }
                }
            }
        }
    };

    contents
        .lines()
        .enumerate()
        .map(|(i, line)| parse_line(line, i))
        .flatten()
        .for_each(|num| calculate(&num));

    println!("{}", sum)
}
