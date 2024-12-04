use std::fs;

#[derive(Debug)]
struct MapLayer {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl MapLayer {
    fn new(line: &str) -> MapLayer {
        let [destination_range_start, source_range_start, range_length] = line
        .split_whitespace()
        .map(|num_str| num_str.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()[..] else { panic!("Can't destructure") };

        MapLayer {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
    fn map_source_to_destination(&self, source: usize) -> Option<usize> {
        if source >= self.source_range_start && source < self.source_range_start + self.range_length
        {
            Some(self.destination_range_start + source - self.source_range_start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    layers: Vec<MapLayer>,
}

impl Map {
    fn new() -> Map {
        Map { layers: vec![] }
    }
    fn add_layer(&mut self, layer: MapLayer) {
        self.layers.push(layer);
    }
    fn map_source_to_destination(&self, source: usize) -> usize {
        for layer in &self.layers {
            if let Some(destination) = &layer.map_source_to_destination(source) {
                return *destination;
            }
        }
        source
    }
}

fn parse_maps(contents: &Vec<&str>) -> Vec<Map> {
    let mut maps = vec![];
    let mut current_map = Map::new();
    for i in 2..contents.len() {
        let line = contents[i];
        if line.is_empty() {
            maps.push(current_map);
            current_map = Map::new();
        } else if !line.contains(":") {
            current_map.add_layer(MapLayer::new(line));
        }
    }
    maps.push(current_map);
    maps
}

fn parse_seeds(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .filter(|num_str| num_str.parse::<usize>().is_ok())
        .map(|num_str| num_str.parse::<usize>().unwrap())
        .map(|num| num)
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|window| window)
        .enumerate()
        .filter(|(i, _window)| i % 2 == 0)
        .map(|(_i, window)| window)
        .into_iter()
        .map(|window| {
            println!("{:?}", window);
            let [source, length] = window[..] else { panic!("Can't destructure") };
            (source..source + length).collect::<Vec<usize>>()
        })
        .flatten()
        .collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    let contents = contents.lines().collect::<Vec<&str>>();

    let maps = parse_maps(&contents);
    let seeds = parse_seeds(&contents[0]);

    let min = seeds
        .iter()
        .map(|seed| {
            maps.iter()
                .fold(*seed, |acc, map| map.map_source_to_destination(acc))
        })
        .min()
        .unwrap();

    println!("{}", min);
}
