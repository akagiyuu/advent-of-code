const INPUT: &str = include_str!("../input.txt");
const TOTAL_SPACE: usize = 5;

use itertools::Itertools;

fn is_lock(raw: &str) -> bool {
    raw.as_bytes()[0] == b'#'
}

fn parse_schematic(raw: &str) -> Vec<usize> {
    let raw: Vec<_> = raw
        .lines()
        .skip(1)
        .take(5)
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let n = raw.len();
    let m = raw[0].len();

    (0..m)
        .map(|j| (0..n).map(|i| raw[i][j]).filter(|&x| x == b'#').count())
        .collect()
}

fn is_valid_pair(lock: &[usize], key: &[usize]) -> bool {
    lock.iter()
        .zip(key.iter())
        .all(|(a, b)| a + b <= TOTAL_SPACE)
}

fn main() {
    let mut locks = vec![];
    let mut keys = vec![];

    for raw in INPUT.split("\n\n") {
        if is_lock(raw) {
            locks.push(parse_schematic(raw));
        } else {
            keys.push(parse_schematic(raw));
        }
    }

    let valid_pair_count = [locks.iter(), keys.iter()]
        .into_iter()
        .multi_cartesian_product()
        .filter(|input| is_valid_pair(input[0], input[1]))
        .count();
    println!("{}", valid_pair_count);
}
