use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Error");
    let mut matrix: Vec<Vec<char>> = vec![];

    content.lines().for_each(|line| {
        let line_vec: Vec<char> = line.chars().collect();

        matrix.push(line_vec);
    });

    part_2(&matrix);
}

fn part_2(matrix: &Vec<Vec<char>>) {
    let mut count = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'A' {
                if is_x_mas_pattern(&matrix, i, j) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn is_x_mas_pattern(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // Border case
    if x == 0 || y == 0 || x == matrix.len() - 1 || y == matrix[0].len() - 1 {
        return false;
    }

    return (matrix[x - 1][y - 1] == 'M' && matrix[x + 1][y + 1] == 'S'
        || matrix[x - 1][y - 1] == 'S' && matrix[x + 1][y + 1] == 'M')
        && (matrix[x - 1][y + 1] == 'M' && matrix[x + 1][y - 1] == 'S'
            || matrix[x - 1][y + 1] == 'S' && matrix[x + 1][y - 1] == 'M');
}

fn part_1(matrix: &Vec<Vec<char>>) {
    let mut count = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                count += search(&matrix, i as i32, j as i32, "XMAS", None);
            }
        }
    }

    println!("{}", count);
}

struct Direction {
    row_offset: i32,
    col_offset: i32,
}

fn get_legal_directions(row: i32, col: i32, matrix: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    let mut directions = Vec::new();

    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (row_offset, col_offset) in offsets {
        let new_row = row + row_offset;
        let new_col = col + col_offset;

        // Check if new position is within bounds
        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            directions.push((new_row, new_col));
        }
    }

    directions
}

fn search(matrix: &Vec<Vec<char>>, x: i32, y: i32, to_find: &str, dir: Option<Direction>) -> i32 {
    if matrix[x as usize][y as usize] != to_find.chars().next().unwrap() {
        return 0;
    }

    let to_find = &to_find[1..];

    if to_find.len() == 0 {
        return 1;
    }

    let mut count = 0;

    match dir {
        Some(dir) => {
            let new_x = x + dir.row_offset;
            let new_y = y + dir.col_offset;

            if new_x >= 0
                && new_x < matrix.len() as i32
                && new_y >= 0
                && new_y < matrix[0].len() as i32
            {
                count += search(matrix, new_x, new_y, to_find, Some(dir));
            }
        }
        None => {
            let directions = get_legal_directions(x, y, matrix);

            directions.iter().for_each(|(i, j)| {
                count += search(
                    matrix,
                    *i,
                    *j,
                    to_find,
                    Some(Direction {
                        row_offset: i - x,
                        col_offset: j - y,
                    }),
                );
            });
        }
    };

    count
}
