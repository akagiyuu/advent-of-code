#![feature(iter_array_chunks)]
const INPUT: &str = include_str!("input.txt");

fn calculate_priority(c: u8) -> u8 {
    if c.is_ascii_lowercase() {
        return c - b'a' + 1;
    }
    if c.is_ascii_uppercase() {
        return c - b'A' + 27;
    }
    0
}

const MAX_PRIORITY: usize = 52;

fn generate_existence_array(a: &str) -> [bool; MAX_PRIORITY + 1] {
    let mut exist = [false; MAX_PRIORITY + 1];
    for c in a.bytes() {
        exist[calculate_priority(c) as usize] = true;
    }
    exist
}

fn get_common_item_priority(rucksack: &str) -> usize {
    let (a, b) = rucksack.split_at(rucksack.len() / 2);

    let a_exist = generate_existence_array(a);
    let b_exist = generate_existence_array(b);

    for i in 0..=MAX_PRIORITY {
        if a_exist[i] && b_exist[i] {
            return i;
        }
    }
    0
}

fn get_common_item_priority_in_group<const N: usize>(group: [&str; N]) -> usize {
    let combined_existence_array = group
        .iter()
        .map(|&rucksack| generate_existence_array(rucksack))
        .fold([true; MAX_PRIORITY + 1], |result, rucksack| {
            let mut result = result;
            for i in 0..=MAX_PRIORITY {
                result[i] = result[i] && rucksack[i];
            }
            result
        });

    for (i, &exist) in combined_existence_array.iter().enumerate() {
        if exist {
            return i;
        }
    }
    0
}

fn main() {
    println!("{}", INPUT.lines().map(get_common_item_priority).sum::<usize>());
    println!(
        "{}",
        INPUT
            .lines()
            .array_chunks::<3>()
            .map(get_common_item_priority_in_group)
            .sum::<usize>()
    );
}
