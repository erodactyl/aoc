use std::fs;

struct Reveal {
    red: u32,
    blue: u32,
    green: u32,
}

struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

trait Feasability {
    fn possible(&self, red: u32, blue: u32, green: u32) -> bool;
}

impl Feasability for Reveal {
    fn possible(&self, red: u32, blue: u32, green: u32) -> bool {
        red >= self.red && blue >= self.blue && green >= self.green
    }
}

impl Feasability for Game {
    fn possible(&self, red: u32, blue: u32, green: u32) -> bool {
        self.reveals
            .iter()
            .all(|rev| rev.possible(red, blue, green))
    }
}

impl Reveal {
    fn set_colors(&mut self, reveal_str: &str) {
        reveal_str.split(",").into_iter().for_each(|color_str| {
            let arr: Vec<&str> = color_str.split(" ").collect();
            let count: u32 = arr[1].parse().unwrap();
            if arr[2] == "red" {
                self.red = count;
            } else if arr[2] == "blue" {
                self.blue = count;
            } else {
                self.green = count;
            }
        })
    }
}

impl Game {
    fn new(line: &str) -> Game {
        let arr: Vec<&str> = line.split(":").collect();
        let id = &arr[0][5..].parse().unwrap();
        let reveal_strings: Vec<&str> = arr[1].split(";").collect();
        let reveals: Vec<Reveal> = reveal_strings
            .iter()
            .map(|str| {
                let mut reveal = Reveal {
                    blue: 0,
                    green: 0,
                    red: 0,
                };
                reveal.set_colors(str);
                reveal
            })
            .collect();
        Game { id: *id, reveals }
    }
    fn get_power(self: Game) -> u32 {
        let red = self.reveals.iter().map(|rev| rev.red).max().unwrap();
        let blue = self.reveals.iter().map(|rev| rev.blue).max().unwrap();
        let green = self.reveals.iter().map(|rev| rev.green).max().unwrap();
        red * blue * green
    }
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path);
    if contents.is_err() {
        println!("Error: {}", contents.err().unwrap());
        return;
    }

    let sum: u32 = contents
        .unwrap()
        .lines()
        .map(|line| Game::new(line))
        .map(|game| game.get_power())
        .sum();

    println!("{}", sum)
}
