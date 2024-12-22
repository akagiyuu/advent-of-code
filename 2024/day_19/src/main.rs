use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn count_different_way<'a>(
    pattern: &'a [u8],
    towels: &HashSet<Vec<u8>>,
    max_length: usize,
    min_length: usize,
    cache: &mut HashMap<&'a [u8], usize>,
) -> usize {
    if let Some(&v) = cache.get(pattern) {
        return v;
    }

    if pattern.is_empty() {
        cache.insert(pattern, 1);
        return 1;
    }

    let n = max_length.min(pattern.len());
    let mut count = 0;

    for i in min_length..=n {
        if !towels.contains(&pattern[..i]) {
            continue;
        }
        count += count_different_way(&pattern[i..], towels, max_length, min_length, cache);
    }

    cache.insert(pattern, count);
    count
}

fn count_possible<'a>(
    patterns: impl Iterator<Item = &'a str>,
    towels: &HashSet<Vec<u8>>,
    max_length: usize,
    min_length: usize,
    cache: &mut HashMap<&'a [u8], usize>,
) -> usize {
    patterns
        .filter(|pattern| {
            count_different_way(pattern.as_bytes(), towels, max_length, min_length, cache) == 0
        })
        .count()
}

fn count_different_way_all<'a>(
    patterns: impl Iterator<Item = &'a str>,
    towels: &HashSet<Vec<u8>>,
    max_length: usize,
    min_length: usize,
    cache: &mut HashMap<&'a [u8], usize>,
) -> usize {
    patterns
        .map(|pattern| {
            count_different_way(pattern.as_bytes(), towels, max_length, min_length, cache)
        })
        .sum()
}

fn main() {
    let (towels, patterns) = INPUT.split_once("\n\n").unwrap();

    let max_length = towels.split(", ").map(|x| x.len()).max().unwrap();
    let min_length = towels.split(", ").map(|x| x.len()).min().unwrap();
    let towels = towels.split(", ").map(|x| x.as_bytes().to_vec()).collect();

    let mut cache = HashMap::new();

    println!(
        "{}",
        count_possible(
            patterns.lines(),
            &towels,
            max_length,
            min_length,
            &mut cache
        )
    );
    println!(
        "{}",
        count_different_way_all(
            patterns.lines(),
            &towels,
            max_length,
            min_length,
            &mut cache
        )
    );
}
