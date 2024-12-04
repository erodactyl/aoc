use std::collections::HashMap;
use std::fs;

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let contents = contents.lines().collect::<Vec<&str>>();

    let rl = contents[0].chars().collect::<Vec<char>>();

    let mut ending_in_a = vec![];

    let mut map = HashMap::new();
    contents.iter().skip(2).for_each(|line| {
        let id = line.get(0..3).unwrap().to_string();

        if id.ends_with('A') {
            ending_in_a.push(id.clone());
        }

        let left = line.get(7..10).unwrap().to_string();
        let right = line.get(12..15).unwrap().to_string();

        map.insert(id, (left, right));
    });

    let mut steps: Vec<usize> = vec![];

    println!("{:?}", ending_in_a);

    for mut location in ending_in_a {
        let mut index = 0;

        let mut next_direction = || {
            let current = rl[index];
            if index < rl.len() - 1 {
                index += 1;
            } else {
                index = 0;
            }
            current
        };

        let mut _steps = 0;

        while !location.ends_with('Z') {
            _steps += 1;
            let (left, right) = map.get(&location).unwrap();
            let direction = next_direction();
            if direction == 'L' {
                location = left.clone();
            } else {
                location = right.clone();
            }
        }

        steps.push(_steps);
    }

    let res = steps.iter().fold(1, |acc, curr| lcm(*curr, acc));

    println!("{}", res);
}
