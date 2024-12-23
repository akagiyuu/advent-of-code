#![feature(iter_map_windows)]

use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn mod_pow_2(x: u64, p: u64) -> u64 {
    ((1 << p) - 1) & x
}

fn get_next(x: u64) -> u64 {
    let x = x ^ mod_pow_2(x << 6, 24);
    let x = x ^ (x >> 5);
    x ^ mod_pow_2(x << 11, 24)
}

fn iterate_max_price(secret: u64, iter: usize) -> impl Iterator<Item = ([i8; 4], i64)> {
    let mut cache = HashMap::new();
    let mut cur = secret;

    (0..iter)
        .map(move |_| {
            cur = get_next(cur);
            cur
        })
        .map(|x| (x % 10) as i8)
        .map_windows(|window: &[i8; 5]| *window)
        .for_each(|window| {
            let mut key = [0; 4];
            for i in 0..4 {
                key[i] = window[i + 1] - window[i];
            }
            let value = window[4] as i64;
            cache.entry(key).or_insert(value);
        });

    cache.into_iter()
}

fn get_sum_secret(secrets: impl Iterator<Item = u64>, iter: usize) -> u64 {
    secrets
        .map(|secret| (0..iter).fold(secret, |secret, _| get_next(secret)))
        .sum()
}

fn get_max_price(secrets: impl Iterator<Item = u64>, iter: usize) -> i64 {
    let mut cache = HashMap::<[i8; 4], i64>::new();
    for secret in secrets {
        for (key, value) in iterate_max_price(secret, iter) {
            *cache.entry(key).or_default() += value;
        }
    }

    *cache.values().max().unwrap()
}

fn main() {
    let secrets = INPUT.lines().map(|line| line.parse::<u64>().unwrap());

    println!("{}", get_sum_secret(secrets.clone(), 2000));
    println!("{}", get_max_price(secrets, 2000));
}
