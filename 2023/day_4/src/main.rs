use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn get_matched_number_count(card: &str) -> usize {
    let mut card_iter = card.chars();
    let mut matched_count = 0;
    let mut current_section = 0;
    let mut winning_numbers = HashSet::new();

    for char in card_iter.by_ref() {
        if char == ':' {
            break;
        }
    }
    let mut temp_value = 0;
    for char in card_iter.by_ref() {
        match char {
            '|' => current_section += 1,
            char if char.is_ascii_digit() => {
                temp_value = temp_value * 10 + char.to_digit(10).unwrap() as usize;
            }
            _ => {
                if temp_value == 0 {
                    continue;
                }
                if current_section == 0 {
                    winning_numbers.insert(temp_value);
                } else if winning_numbers.contains(&temp_value) {
                    matched_count += 1;
                }
                temp_value = 0;
            }
        }
    }
    if temp_value > 0 && winning_numbers.contains(&temp_value) {
        matched_count += 1;
    }
    matched_count
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
