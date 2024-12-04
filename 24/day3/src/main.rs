use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Error");

    let mut sum = 0;
    let mut enabled = true;

    content.lines().for_each(|line| {
        let re =
            Regex::new(r"(mul\((?<left>[0-9]{1,3}),(?<right>[0-9]{1,3})\))|(do\(\))|(don\'t\(\))")
                .unwrap();
        re.captures_iter(line).for_each(|cmd| {
            // 1, 2, 3 are mul (first capture group), left, and right
            // 4 is do
            // 5 is don't

            if cmd.get(4).is_some() {
                enabled = true;
            } else if cmd.get(5).is_some() {
                enabled = false;
            } else if enabled {
                let left: i32 = cmd.name("left").unwrap().as_str().parse().unwrap();
                let right: i32 = cmd.name("right").unwrap().as_str().parse().unwrap();
                sum += left * right;
            }
        });
    });

    println!("{}", sum);
}
