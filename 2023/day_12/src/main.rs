use std::{collections::HashMap, fmt::Debug};

use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

struct PossibleDamagedSpringGroup {
    damaged: usize,
    unknown: usize,
}
impl Debug for PossibleDamagedSpringGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.damaged, self.unknown)
    }
}

fn get_damaged_spring_groups(record: &[u8]) -> Vec<usize> {
    let mut damaged_spring_counts = vec![];
    let mut count = 0;
    for c in record {
        match c {
            b'#' => count += 1,
            b'.' => {
                if count == 0 {
                    continue;
                }
                damaged_spring_counts.push(count);
                count = 0;
            }
            _ => unreachable!(),
        }
    }
    if count != 0 {
        damaged_spring_counts.push(count);
    }
    damaged_spring_counts
}

fn count_possible_arrangement(
    record: &str,
    damaged_spring_counts: &[usize],
    current_damaged_spring_count: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if record.is_empty() {
        return if damaged_spring_counts.len() == 0 {
            1
        } else {
            0
        };
    }
    if let Some(&possible_arrangement) = cache.get(&(
        damaged_spring_counts.len(),
        current_damaged_spring_count,
        record.len(),
    )) {
        return possible_arrangement;
    }
    let possible_arrangement = match record.as_bytes()[0] {
        b'.' => {
            if current_damaged_spring_count == 0 {
                count_possible_arrangement(
                    &record[1..],
                    damaged_spring_counts,
                    current_damaged_spring_count,
                    cache,
                )
            } else if damaged_spring_counts.is_empty() {
                0
            } else if current_damaged_spring_count == damaged_spring_counts[0] {
                count_possible_arrangement(&record[1..], &damaged_spring_counts[1..], 0, cache)
            } else {
                0
            }
        }
        b'#' => count_possible_arrangement(
            &record[1..],
            damaged_spring_counts,
            current_damaged_spring_count + 1,
            cache,
        ),
        b'?' => {
            count_possible_arrangement(
                &record[1..],
                damaged_spring_counts,
                current_damaged_spring_count + 1,
                cache,
            ) + if current_damaged_spring_count == 0 {
                count_possible_arrangement(&record[1..], damaged_spring_counts, 0, cache)
            } else if damaged_spring_counts.is_empty() {
                0
            } else if current_damaged_spring_count == damaged_spring_counts[0] {
                count_possible_arrangement(&record[1..], &damaged_spring_counts[1..], 0, cache)
            } else {
                0
            }
        }
        _ => unreachable!(),
    };
    cache.insert(
        (
            damaged_spring_counts.len(),
            current_damaged_spring_count,
            record.len(),
        ),
        possible_arrangement,
    );
    possible_arrangement
}

fn main() {
    let possible_arrangement_sum = INPUT
        .lines()
        .par_bridge()
        .map(|line| {
            let (record, damaged_spring_counts) = line.split_once(' ').unwrap();
            let mut record = record.to_string();
            record.push('.');

            let damaged_spring_counts = damaged_spring_counts
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut cache = HashMap::new();

            count_possible_arrangement(&record, &damaged_spring_counts, 0, &mut cache)
        })
        .sum::<usize>();
    println!("{}", possible_arrangement_sum);

    const LOOP_AMOUNT: usize = 5;
    let possible_arrangement_sum = INPUT
        .lines()
        .par_bridge()
        .map(|line| {
            let (record_raw, damaged_spring_counts_raw) = line.split_once(' ').unwrap();

            let damaged_spring_counts_raw = damaged_spring_counts_raw
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut damaged_spring_counts =
                Vec::<usize>::with_capacity(damaged_spring_counts_raw.len() * LOOP_AMOUNT);
            for _ in 0..LOOP_AMOUNT {
                damaged_spring_counts.extend(damaged_spring_counts_raw.iter());
            }

            let mut record = String::with_capacity(record_raw.len() * LOOP_AMOUNT);
            for _ in 0..LOOP_AMOUNT {
                record.push_str(record_raw);
                record.push('?');
            }
            record.pop();
            record.push('.');

            let mut cache = HashMap::new();

            count_possible_arrangement(&record, &damaged_spring_counts, 0, &mut cache)
        })
        .sum::<usize>();
    println!("{}", possible_arrangement_sum);
}
