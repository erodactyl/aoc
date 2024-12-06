use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Error");
    let mut matrix: Vec<Vec<char>> = vec![];

    content.lines().for_each(|line| {
        let line_vec: Vec<char> = line.chars().collect();

        matrix.push(line_vec);
    });

    let mut guard_location: (i32, i32) = (0, 0);
    let mut guard_direction = '^';

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                guard_location = (i as i32, j as i32);
            }
        }
    }

    let init_guard_location = guard_location;

    // Part 1
    let mut steps = 1;

    let mut directions: HashMap<char, (i32, i32)> = HashMap::new();

    directions.insert('^', (-1, 0));
    directions.insert('v', (1, 0));
    directions.insert('>', (0, 1));
    directions.insert('<', (0, -1));

    loop {
        let dir = directions.get(&guard_direction).unwrap();
        let next_location = (guard_location.0 + dir.0, guard_location.1 + dir.1);

        if next_location.0 < 0
            || next_location.0 >= matrix.len() as i32
            || next_location.1 < 0
            || next_location.1 >= matrix[0].len() as i32
        {
            // Guard left the room
            break;
        }

        if matrix[next_location.0 as usize][next_location.1 as usize] == '#' {
            guard_direction = match guard_direction {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Invalid direction"),
            };
        } else {
            guard_location = next_location;
            if matrix[guard_location.0 as usize][guard_location.1 as usize] != 'X' {
                steps += 1;
                matrix[guard_location.0 as usize][guard_location.1 as usize] = 'X';
                // Mark the path
            }
        }
    }

    println!("Steps: {}", steps);

    // Part 2

    let mut infinite_walks = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if (i as i32, j as i32) != init_guard_location && matrix[i][j] == 'X' {
                matrix[i][j] = '#';
                if is_infinite_walk(&matrix, init_guard_location) {
                    println!("Infinite walk found at: ({}, {})", i, j);
                    infinite_walks += 1;
                }
                matrix[i][j] = 'X';
            }
        }
    }

    println!("Infinite walks: {}", infinite_walks);
}

fn is_infinite_walk(matrix: &Vec<Vec<char>>, init_location: (i32, i32)) -> bool {
    let mut directions: HashMap<char, (i32, i32)> = HashMap::new();

    directions.insert('^', (-1, 0));
    directions.insert('v', (1, 0));
    directions.insert('>', (0, 1));
    directions.insert('<', (0, -1));

    let mut guard_location = init_location;
    let mut guard_direction = '^';

    let mut all_steps: HashSet<(i32, i32, char)> = HashSet::new();

    loop {
        let dir = directions.get(&guard_direction).unwrap();
        let next_location = (guard_location.0 + dir.0, guard_location.1 + dir.1);

        if next_location.0 < 0
            || next_location.0 >= matrix.len() as i32
            || next_location.1 < 0
            || next_location.1 >= matrix[0].len() as i32
        {
            // Guard left the room
            break;
        }

        if matrix[next_location.0 as usize][next_location.1 as usize] == '#' {
            guard_direction = match guard_direction {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Invalid direction"),
            };
        } else {
            guard_location = next_location;
        }

        if all_steps.contains(&(guard_location.0, guard_location.1, guard_direction)) {
            return true;
        } else {
            all_steps.insert((guard_location.0, guard_location.1, guard_direction));
        }
    }

    return false;
}
