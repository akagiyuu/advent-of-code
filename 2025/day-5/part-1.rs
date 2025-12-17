use std::{
    cmp::Ordering,
    io::{stdin, Read},
};

fn merge(mut segments: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    segments.sort();

    let mut res = Vec::with_capacity(segments.len());
    let (mut start, mut end) = segments[0];

    for (n_start, n_end) in segments.into_iter().skip(1) {
        if !(start..=end).contains(&n_start) {
            res.push((start, end));

            start = n_start;
            end = n_end;

            continue;
        }

        end = end.max(n_end);
    }

    res.push((start, end));

    res
}

fn contain(x: u64, segments: &[(u64, u64)]) -> bool {
    eprintln!("DEBUGPRINT[20]: {}:{}: x={:#?}", file!(), line!(), x);
    let i = segments
        .binary_search_by(|element| match element.0.cmp(&x) {
            Ordering::Equal => Ordering::Less,
            ord => ord,
        })
        .unwrap_err();
    if i == 0 {
        return false;
    }
    let (start, end) = segments[i - 1];
    // println!("{x} -> [{start}, {end}]");

    (start..=end).contains(&x)
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let (segments, ids) = buffer.split_once("\n\n").unwrap();

    let segments = segments
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();

            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect();
    let segments = merge(segments);
    // eprintln!(
    //     "DEBUGPRINT[22]: {}:{}: segments={:#?}",
    //     file!(),
    //     line!(),
    //     segments
    // );

    let ids = ids.lines().map(|x| x.parse::<u64>().unwrap());

    let mut cnt = 0;
    for id in ids {
        if contain(id, &segments) {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
