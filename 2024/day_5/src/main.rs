use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn is_valid_update(update: &[usize], rules: &HashMap<usize, u128>) -> bool {
    let mut acc = 0;
    for x in update {
        match rules.get(x) {
            Some(behind) if acc & behind != 0 => return false,
            _ => {}
        }
        acc |= 1 << x;
    }

    true
}

fn sum_valid_update(updates: &[Vec<usize>], rules: &HashMap<usize, u128>) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            if is_valid_update(update, rules) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn sort(update: &[usize], rules: &HashMap<usize, u128>) -> Vec<usize> {
    let mut sorted_update = update.to_vec();

    let xor_all = update.iter().fold(0, |acc, x| acc | (1 << x));

    for &x in update {
        let element_behind_count = (rules.get(&x).unwrap_or(&0) & xor_all).count_ones();
        let i = update.len() - 1 - element_behind_count as usize;
        sorted_update[i] = x;
    }

    sorted_update
}

fn sum_invalid_upadte(updates: &[Vec<usize>], rules: &HashMap<usize, u128>) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            if is_valid_update(update, rules) {
                None
            } else {
                let sorted_update = sort(update, rules);
                Some(sorted_update[sorted_update.len() / 2])
            }
        })
        .sum()
}

fn main() {
    let (rules_raw, updates_raw) = INPUT.split_once("\n\n").unwrap();

    let mut rules = HashMap::<usize, u128>::new();

    for rule in rules_raw.lines() {
        let [a, b]: [usize; 2] = rule
            .split('|')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        *rules.entry(a).or_insert(0) |= 1 << b;
    }

    let updates: Vec<Vec<usize>> = updates_raw
        .lines()
        .map(|update| update.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    println!("{}", sum_valid_update(&updates, &rules));
    println!("{}", sum_invalid_upadte(&updates, &rules));
}
