use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn get_matched_number_count(card: &str) -> usize {
    let (_, card) = card.split_once(": ").unwrap();
    let (winning_numbers, numbers) = card.split_once(" | ").unwrap();
    let winning_numbers = winning_numbers
        .split(' ')
        .filter_map(|number| number.trim().parse::<usize>().ok())
        .collect::<HashSet<_>>();
    numbers
        .split(' ')
        .filter_map(|number| number.trim().parse::<usize>().ok())
        .filter(|number| winning_numbers.contains(number))
        .count()
}

fn get_total_card_earned(cards: &str) -> usize {
    let card_count = cards.lines().count();
    let mut cards_copy_count = vec![1; card_count];
    for (i, card) in cards.lines().enumerate() {
        let matched_count = get_matched_number_count(card);
        for j in 1..=matched_count {
            if i + j >= card_count {
                break;
            }
            cards_copy_count[i + j] += cards_copy_count[i];
        }
    }
    cards_copy_count.iter().sum()
}

fn main() {
    let point_sum = INPUT
        .lines()
        .map(|card| match get_matched_number_count(card) {
            0 => 0,
            matched_count => 1 << (matched_count - 1),
        })
        .sum::<usize>();
    println!("{}", point_sum);
    println!("{}", get_total_card_earned(INPUT));
}
