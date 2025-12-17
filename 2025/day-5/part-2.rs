use std::{
    cmp::Ordering,
    io::{stdin, Read},
};

fn mersure(mut segments: Vec<(u64, u64)>) -> u64 {
    segments.sort();

    let mut size = 0;
    let (mut start, mut end) = segments[0];

    for (n_start, n_end) in segments.into_iter().skip(1) {
        if !(start..=end).contains(&n_start) {
            size += end - start + 1;

            start = n_start;
            end = n_end;

            continue;
        }

        end = end.max(n_end);
    }

    size += end - start + 1;

    size
}

fn main() {
    let mut segments = String::new();
    stdin().read_to_string(&mut segments).unwrap();

    let segments = segments
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();

            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect();

    println!("{}", mersure(segments));
}
