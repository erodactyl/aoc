use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn get_hand_details(hand_str: &str) -> HandType {
    let mut hand_map: HashMap<char, usize> = HashMap::new();

    let mut js = 0;

    for card in hand_str.chars() {
        if card == 'J' {
            js += 1;
        } else {
            let count = hand_map.entry(card).or_insert(0);
            *count += 1;
        }
    }

    let mut counts = vec![];

    for (_, count) in hand_map {
        counts.push(count);
    }
    counts.sort();
    counts.reverse();

    if counts.len() == 0 {
        counts.push(0);
    }
    counts[0] += js;

    if counts[0] == 5 {
        HandType::FiveOfAKind
    } else if counts[0] == 4 {
        HandType::FourOfAKind
    } else if counts[0] == 3 {
        if counts[1] == 2 {
            HandType::FullHouse
        } else {
            HandType::ThreeOfAKind
        }
    } else if counts[0] == 2 {
        if counts[1] == 2 {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
    } else {
        HandType::HighCard
    }
}

fn cmp_cards(this: char, other: char) -> Ordering {
    let self_strength = get_card_strength(this);
    let other_strength = get_card_strength(other);
    if self_strength > other_strength {
        Ordering::Greater
    } else if self_strength < other_strength {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn cmp_hands(this: &str, other: &str) -> Ordering {
    let self_details = get_hand_details(this);
    let other_details = get_hand_details(other);
    let self_strength = get_hand_strength(&self_details);
    let other_strength = get_hand_strength(&other_details);
    if self_strength > other_strength {
        return Ordering::Greater;
    } else if self_strength < other_strength {
        return Ordering::Less;
    }

    for i in 0..5 {
        let res = cmp_cards(this.chars().nth(i).unwrap(), other.chars().nth(i).unwrap());
        if res != Ordering::Equal {
            return res;
        }
    }

    panic!("cmp_hands failed");
}

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand_strength(hand: &HandType) -> usize {
    match hand {
        HandType::HighCard => 1,
        HandType::OnePair => 2,
        HandType::TwoPair => 3,
        HandType::ThreeOfAKind => 4,
        HandType::FullHouse => 5,
        HandType::FourOfAKind => 6,
        HandType::FiveOfAKind => 7,
    }
}

fn get_card_strength(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0,
        'T' => 10,
        d => d.to_digit(10).unwrap(),
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error reading file");

    let mut hands = contents.lines().map(|line| {
        let [hand, bid] = line.split_whitespace().collect::<Vec<&str>>()[..] else { panic!("WTF") };
        (hand, bid.parse::<usize>().unwrap())
    }).collect::<Vec<(&str, usize)>>();

    hands.sort_by(|a, b| cmp_hands(a.0, b.0));

    let result: usize = hands
        .iter()
        .enumerate()
        .map(|(idx, (hand, bid))| (idx + 1) * bid)
        .sum();

    println!("{}", result);
}
