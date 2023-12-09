#![feature(iter_map_windows)]
const INPUT: &str = include_str!("input.txt");

fn find_next(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|&element| element == 0) {
        return 0;
    }
    let last = sequence[sequence.len() - 1];
    let diff = sequence
        .iter()
        .map_windows(|&[a, b]| b - a)
        .collect::<Vec<_>>();
    last + find_next(&diff)
}
fn find_prev(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|&element| element == 0) {
        return 0;
    }
    let first = sequence[0];
    let diff = sequence
        .iter()
        .map_windows(|&[a, b]| b - a)
        .collect::<Vec<_>>();
    first - find_prev(&diff)
}

fn find_prev_and_next(sequence: &[i64]) -> (i64, i64) {
    if sequence.iter().all(|&element| element == 0) {
        return (0, 0);
    }
    let first = sequence[0];
    let last = sequence[sequence.len() - 1];
    let diff = sequence
        .iter()
        .map_windows(|&[a, b]| b - a)
        .collect::<Vec<_>>();
    let (a, b) = find_prev_and_next(&diff);
    (first - a, last + b)
}

fn main() {
    let sums = INPUT
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|sequence| find_prev_and_next(&sequence))
        .fold((0, 0), |acc, (prev, next)| (acc.0 + prev, acc.1 + next));
    println!("{}", sums.1);
    println!("{}", sums.0);
}
