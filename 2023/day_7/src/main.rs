use std::{cmp::Ordering, ops::Deref};

const INPUT: &str = include_str!("input.txt");

const MAX_CARD: usize = 12;
fn encode(c: char) -> usize {
    match c {
        c if c.is_ascii_digit() => (c as u8 - b'2') as usize,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!(),
    }
}
fn encode_part2(c: char) -> usize {
    match c {
        'J' => 0,
        c if c.is_ascii_digit() => (c as u8 - b'1') as usize,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!(),
    }
}

fn compare_hand(a: &str, b: &str, encode: impl Fn(char) -> usize) -> Ordering {
    for (c_a, c_b) in a.chars().zip(b.chars()) {
        if c_a != c_b {
            return encode(c_a).cmp(&encode(c_b));
        }
    }
    Ordering::Equal
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl HandKind {
    fn get_kind(hand: &str) -> Self {
        let mut frequency = [0; MAX_CARD + 1];
        for c in hand.chars() {
            frequency[encode(c)] += 1;
        }
        let mut frequency = frequency
            .iter()
            .filter(|&&frequency| frequency > 0)
            .collect::<Vec<_>>();
        frequency.sort();
        match frequency.deref() {
            [5] => HandKind::FiveOfAKind,
            [1, 4] => HandKind::FourOfAKind,
            [2, 3] => HandKind::FullHouse,
            [1, 1, 3] => Self::ThreeOfAKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => panic!(),
        }
    }
    fn get_highest_possible_kind(hand: &str) -> Self {
        let mut frequency = [0; MAX_CARD + 1];
        for c in hand.chars() {
            frequency[encode(c)] += 1;
        }
        let joker_count = frequency[encode('J')];
        frequency[encode('J')] = 0;
        let mut frequency = frequency
            .iter()
            .filter(|&&frequency| frequency > 0).copied()
            .collect::<Vec<_>>();
        frequency.sort();
        let Some(last) = frequency.iter_mut().last() else {
            return HandKind::FiveOfAKind;
        };
        *last += joker_count;
        match frequency.deref() {
            [5] => HandKind::FiveOfAKind,
            [1, 4] => HandKind::FourOfAKind,
            [2, 3] => HandKind::FullHouse,
            [1, 1, 3] => Self::ThreeOfAKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => panic!(),
        }
    }
}

fn main() {
    let mut hands = INPUT
        .lines()
        .map(|line| {
            let hand = &line[0..5];
            let bid = line[5..].trim().parse::<usize>().unwrap();
            let kind = HandKind::get_kind(hand);
            (hand, bid, kind)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| {
        let cmp = a.2.cmp(&b.2);
        if cmp == Ordering::Equal {
            return compare_hand(a.0, b.0, encode);
        }
        cmp
    });
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.1 * (i + 1));
    println!("{}", total_winnings);

    let mut hands = INPUT
        .lines()
        .map(|line| {
            let hand = &line[0..5];
            let bid = line[5..].trim().parse::<usize>().unwrap();
            let kind = HandKind::get_highest_possible_kind(hand);
            (hand, bid, kind)
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| {
        let cmp = a.2.cmp(&b.2);
        if cmp == Ordering::Equal {
            return compare_hand(a.0, b.0, encode_part2);
        }
        cmp
    });
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.1 * (i + 1));
    println!("{}", total_winnings);
}
