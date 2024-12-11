use std::{
    cmp::Ordering,
    collections::HashMap,
};

const INPUT: &str = include_str!("../input.txt");

const fn generate_power_of_ten<const N: usize>() -> [u64; N] {
    let mut powers = [1; N];
    let mut i = 1;

    while i < N {
        powers[i] = powers[i - 1] * 10;
        i += 1;
    }

    powers
}

const POWERS: [u64; 20] = generate_power_of_ten();

fn upper_bound<T: Ord>(a: &[T], value: T) -> usize {
    a.binary_search_by(|element| match element.cmp(&value) {
        Ordering::Equal => Ordering::Less,
        ord => ord,
    })
    .unwrap_err()
}

fn digit_count(n: u64) -> usize {
    upper_bound(&POWERS, n)
}

fn operate(n: u64) -> (u64, Option<u64>) {
    if n == 0 {
        return (1, None);
    }

    let count = digit_count(n);
    if count % 2 == 1 {
        return (n * 2024, None);
    };

    let left = n / POWERS[count / 2];
    let right = n % POWERS[count / 2];

    (left, Some(right))
}

fn count_stone(mut cur: HashMap<u64, usize>, iter_count: usize) -> usize {
    for _ in 0..iter_count {
        let mut next = HashMap::new();

        for (n, count) in cur {
            let (a, b) = operate(n);

            *next.entry(a).or_default() += count;

            if let Some(b) = b {
                *next.entry(b).or_default() += count;
            }
        }

        cur = next;
    }

    cur.values().sum()
}

fn main() {
    let mut cur = HashMap::new();

    for x in INPUT.split_whitespace() {
        *cur.entry(x.parse().unwrap()).or_default() += 1;
    }

    println!("{}", count_stone(cur.clone(), 25));
    println!("{}", count_stone(cur.clone(), 75));
}
