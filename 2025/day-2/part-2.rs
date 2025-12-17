use std::{cmp::Ordering, collections::BTreeSet, io::stdin};

const N_DIGIT: u32 = 12;
const N: u64 = 1_000_000_000_000;

fn build_invalid() -> Vec<u64> {
    let mut res = BTreeSet::<u64>::new();

    let mut pw = 1;
    for _ in 1..=(N_DIGIT / 2) {
        let next = pw * 10;
        for x in pw..next {
            let mut current = x;
            while current < N {
                current = current * next + x;
                res.insert(current);
            }
        }
        pw = next;
    }

    res.into_iter().collect()
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
    stdin().read_line(&mut input).unwrap();
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
