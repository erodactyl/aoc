use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Error");

    let mut depends_on: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = vec![];

    let mut read_ordering_rules = true;

    content.lines().for_each(|line| {
        if line.is_empty() {
            read_ordering_rules = false
        } else if read_ordering_rules {
            let parts = line
                .split("|")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            if !depends_on.contains_key(&parts[1]) {
                depends_on.insert(parts[1], vec![parts[0]]);
            } else {
                let mut current = depends_on.get(&parts[1]).unwrap().clone();
                current.push(parts[0]);
                depends_on.insert(parts[1], current);
            }
        } else {
            let parts = line
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            updates.push(parts);
        }
    });

    let mut sum = 0;

    let mut invalid_updates: Vec<Vec<i32>> = vec![];

    updates.iter().for_each(|update| {
        if is_update_valid(update, &depends_on) {
            sum += update[update.len() / 2];
        } else {
            invalid_updates.push(update.clone());
        }
    });

    // Part 1
    println!("Sum: {}", sum);

    // Part 2
    let mut sum = 0;
    invalid_updates.iter_mut().for_each(|update| {
        reorder_invalid_update(update, &depends_on);
        sum += update[update.len() / 2];
    });

    println!("Sum: {}", sum);
}

fn reorder_invalid_update(update: &mut Vec<i32>, depends_on: &HashMap<i32, Vec<i32>>) {
    update.sort_by(|a, b| {
        if depends_on.contains_key(a) {
            let a_depends_on = depends_on.get(a).unwrap();
            if a_depends_on.contains(b) {
                return std::cmp::Ordering::Greater;
            }
        }
        if depends_on.contains_key(b) {
            let b_depends_on = depends_on.get(b).unwrap();
            if b_depends_on.contains(a) {
                return std::cmp::Ordering::Less;
            }
        }
        return std::cmp::Ordering::Equal;
    })
}

fn is_update_valid(update: &Vec<i32>, depends_on: &HashMap<i32, Vec<i32>>) -> bool {
    let to_visit = update.iter().collect::<HashSet<&i32>>();
    let mut visited: HashSet<&i32> = HashSet::new();

    for i in 0..update.len() {
        if depends_on.contains_key(&update[i]) {
            let current_depends_on = depends_on.get(&update[i]).unwrap();

            for j in 0..current_depends_on.len() {
                if to_visit.contains(&current_depends_on[j])
                    && !visited.contains(&current_depends_on[j])
                {
                    return false;
                }
            }
        }

        visited.insert(&update[i]);
    }

    return true;
}
