use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn symbol_to_direction(previous: &Dir, symbol: char) -> Dir {
    match (previous, symbol) {
        (Dir::S, '|') => Dir::S,
        (Dir::S, 'J') => Dir::W,
        (Dir::S, 'L') => Dir::E,
        (Dir::W, '-') => Dir::W,
        (Dir::W, 'F') => Dir::S,
        (Dir::W, 'L') => Dir::N,
        (Dir::E, 'J') => Dir::N,
        (Dir::E, '7') => Dir::S,
        (Dir::E, '-') => Dir::E,
        (Dir::N, '|') => Dir::N,
        (Dir::N, 'F') => Dir::E,
        (Dir::N, '7') => Dir::W,
        _ => {
            println!("{:?}, {:?}", previous, symbol);
            panic!();
        }
    }
}

fn count_inside_pipes(grid: &Vec<Vec<char>>, pipes: &HashSet<(usize, usize)>) -> usize {
    let mut count = 0;

    for row in 0..grid.len() {
        let mut should_count = false;

        let mut column = 0;
        while column < grid[row].len() {
            let cell = (row, column);
            // let symbol = grid[row][column];

            // println!("Cell: {:?}, Symbol: {}", cell, symbol);

            if pipes.contains(&cell) {
                should_count = !should_count;
                if pipes.contains(&(row, column + 1)) {
                    while pipes.contains(&(row, column + 1)) {
                        column += 1;
                    }
                    continue;
                }
            } else if should_count {
                println!("Counting {:?}", cell);
                count += 1;
            }

            // println!("\n");
            column += 1;
        }
    }

    count
}

fn main() {
    let mut s_location = (0, 0);

    let content = fs::read_to_string("test.txt")
        .expect("Error reading file")
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            let characters = line.chars();
            characters.enumerate().for_each(|(column_index, char)| {
                if char == 'S' {
                    s_location = (row_index, column_index);
                }
            });
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut pipes = HashSet::new();

    pipes.insert(s_location);

    let mut current = (s_location.0 + 1, s_location.1);
    let mut dir = Dir::S;

    while current != s_location {
        pipes.insert(current);
        dir = symbol_to_direction(&dir, content[current.0][current.1]);
        current = match dir {
            Dir::E => (current.0, current.1 + 1),
            Dir::W => (current.0, current.1 - 1),
            Dir::N => (current.0 - 1, current.1),
            Dir::S => (current.0 + 1, current.1),
        };
    }

    let len = count_inside_pipes(&content, &pipes);

    println!("{}", len)
}
