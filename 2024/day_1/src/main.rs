use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn sum_distance(mut first: Vec<usize>, mut second: Vec<usize>) -> usize {
    first.sort();
    second.sort();

    first
        .into_iter()
        .zip(second)
        .map(|(first, second)| first.abs_diff(second))
        .sum()
}

fn similarity(freq_first: HashMap<usize, usize>, freq_second: HashMap<usize, usize>) -> usize {
    let mut res = 0;

    for (id, count_first) in freq_first {
        res += id * count_first * freq_second.get(&id).copied().unwrap_or(0);
    }

    res
}

fn main() {
    // let (first, second): (Vec<_>, Vec<_>) = INPUT
    //     .lines()
    //     .map(|line| {
    //         let (first, second) = line.split_once("   ").unwrap();
    //         let first = first.parse::<usize>().unwrap();
    //         let second = second.parse::<usize>().unwrap();
    //         (first, second)
    //     })
    //     .unzip();
    //
    // println!("{}", sum_distance(first, second));

    let mut freq_first = HashMap::new();
    let mut freq_second = HashMap::new();

    for line in INPUT.lines() {
        let (first, second) = line.split_once("   ").unwrap();
        let first = first.parse::<usize>().unwrap();
        let second = second.parse::<usize>().unwrap();
        *freq_first.entry(first).or_insert(0) += 1;
        *freq_second.entry(second).or_insert(0) += 1;
    }

    println!("{}", similarity(freq_first, freq_second));
}
