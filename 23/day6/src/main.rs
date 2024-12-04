struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn count_possible_wins(&self) -> usize {
        (1..self.time).fold(0, |acc, speed| {
            if speed * (self.time - speed) > self.distance {
                acc + 1
            } else {
                acc
            }
        })
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("Error reading file");
    let [time, distance] = content
        .lines()
        .map(|line| {
            line.split(":").collect::<Vec<&str>>()[1].replace(" ", "").parse::<usize>().unwrap()
        })
        .collect::<Vec<usize>>()[..] else { panic!("could not destrucutre") };

    let race = Race { time, distance };

    let res = race.count_possible_wins();

    println!("{:?}", res)
}
