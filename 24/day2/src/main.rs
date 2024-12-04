use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut safe_lines_count = 0;

    contents.lines().for_each(|line| {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_line_safe(&numbers) {
            safe_lines_count += 1;
            println!("{:?}: Safe", numbers);
        } else {
            println!("{:?}: Unsafe", numbers);
        }
    });

    println!("Safe lines count: {}", safe_lines_count);
}

fn is_line_safe(line: &Vec<i32>) -> bool {
    // Try both increasing and decreasing sequences
    is_sequence_valid(line, true, true) || is_sequence_valid(line, false, true)
}

fn is_sequence_valid(line: &Vec<i32>, increasing: bool, allow_unsafe: bool) -> bool {
    for i in 1..line.len() {
        let diff = line[i] - line[i - 1];
        let valid_diff = if increasing {
            diff >= 1 && diff <= 3
        } else {
            diff <= -1 && diff >= -3
        };

        if !valid_diff {
            if allow_unsafe {
                // Try removing either number that caused the violation
                let mut new_line_curr = line.clone();
                new_line_curr.remove(i);
                let mut new_line_prev = line.clone();
                new_line_prev.remove(i - 1);

                return is_sequence_valid(&new_line_curr, increasing, false)
                    || is_sequence_valid(&new_line_prev, increasing, false);
            }
            return false;
        }
    }
    true
}
