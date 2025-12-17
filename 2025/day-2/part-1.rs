use std::{cmp::Ordering, io::stdin};

const N: u64 = 1000000;

fn double(mut n: u64) -> u64 {
    let mut first = n;
    let second = n;
    while n > 0 {
        first *= 10;
        n /= 10;
    }
    return first + second;
}

fn build_invalid() -> Vec<u64> {
    (0..N).map(double).collect()
}

fn lower_bound(a: &[u64], x: u64) -> usize {
    a.binary_search_by(|element| match element.cmp(&x) {
        Ordering::Equal => Ordering::Greater,
        ord => ord,
    })
    .unwrap_err()
}

fn upper_bound(a: &[u64], x: u64) -> usize {
    a.binary_search_by(|element| match element.cmp(&x) {
        Ordering::Equal => Ordering::Less,
        ord => ord,
    })
    .unwrap_err()
}

fn main() {
    let invalids = build_invalid();

    let mut res = 0;

    let mut input = String::new();
    stdin().read_line(&mut input);
    for range in input.split(",") {
        let (left, right) = range.split_once('-').unwrap();
        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();

        for i in lower_bound(&invalids, left)..upper_bound(&invalids, right) {
            res += invalids[i];
        }
    }

    println!("{res}");
}
