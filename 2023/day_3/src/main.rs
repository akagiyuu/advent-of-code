use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn is_symbol(char: u8) -> bool {
    !char.is_ascii_digit() && char != b'.'
}

fn part_number_sum(engine_scheme: &[&str]) -> usize {
    let row_count = engine_scheme.len();
    let column_count = engine_scheme[0].len();
    let mut sum = 0;

    for i in 0..row_count {
        let mut j = 0;
        while j < column_count {
            let char = engine_scheme[i].as_bytes()[j];
            if !char.is_ascii_digit() {
                j += 1;
                continue;
            }

            let mut is_part_number = j > 0
                && (is_symbol(engine_scheme[i].as_bytes()[j - 1])
                    || (i > 0 && is_symbol(engine_scheme[i - 1].as_bytes()[j - 1]))
                    || (i + 1 < row_count && is_symbol(engine_scheme[i + 1].as_bytes()[j - 1])));
            let mut k = j;
            while k < column_count && engine_scheme[i].as_bytes()[k].is_ascii_digit() {
                is_part_number |= i > 0 && is_symbol(engine_scheme[i - 1].as_bytes()[k]);
                is_part_number |=
                    i + 1 < row_count && is_symbol(engine_scheme[i + 1].as_bytes()[k]);
                k += 1;
            }
            is_part_number |= k < column_count
                && (is_symbol(engine_scheme[i].as_bytes()[k])
                    || (i > 0 && is_symbol(engine_scheme[i - 1].as_bytes()[k]))
                    || (i + 1 < row_count && is_symbol(engine_scheme[i + 1].as_bytes()[k])));
            if is_part_number {
                sum += engine_scheme[i][j..k].parse::<usize>().unwrap();
            }
            j = k;
        }
    }
    sum
}

fn gear_ratio_sum(engine_scheme: &[&str]) -> usize {
    let row_count = engine_scheme.len();
    let column_count = engine_scheme[0].len();
    let mut gear_parts = HashMap::<(usize, usize), Vec<usize>>::new();

    for i in 0..row_count {
        let mut j = 0;
        while j < column_count {
            let char = engine_scheme[i].as_bytes()[j];
            if !char.is_ascii_digit() {
                j += 1;
                continue;
            }
            let mut adjacent_gears = vec![];

            if j > 0 {
                if is_symbol(engine_scheme[i].as_bytes()[j - 1]) {
                    adjacent_gears.push((i, j - 1));
                }
                if i > 0 && is_symbol(engine_scheme[i - 1].as_bytes()[j - 1]) {
                    adjacent_gears.push((i - 1, j - 1));
                }
                if i + 1 < row_count && is_symbol(engine_scheme[i + 1].as_bytes()[j - 1]) {
                    adjacent_gears.push((i + 1, j - 1));
                }
            }

            let mut k = j;
            while k < column_count && engine_scheme[i].as_bytes()[k].is_ascii_digit() {
                if i > 0 && is_symbol(engine_scheme[i - 1].as_bytes()[k]) {
                    adjacent_gears.push((i - 1, k));
                }
                if i + 1 < row_count && is_symbol(engine_scheme[i + 1].as_bytes()[k]) {
                    adjacent_gears.push((i + 1, k));
                }
                k += 1;
            }
            if k < column_count {
                if is_symbol(engine_scheme[i].as_bytes()[k]) {
                    adjacent_gears.push((i, k));
                }
                if i > 0 && is_symbol(engine_scheme[i - 1].as_bytes()[k]) {
                    adjacent_gears.push((i - 1, k));
                }
                if i + 1 < row_count && is_symbol(engine_scheme[i + 1].as_bytes()[k]) {
                    adjacent_gears.push((i + 1, k));
                }
            }
            let value = engine_scheme[i][j..k].parse::<usize>().unwrap();
            for gear_position in adjacent_gears {
                gear_parts
                    .entry(gear_position)
                    .and_modify(|ratio| ratio.push(value))
                    .or_insert(vec![value]);
            }
            j = k;
        }
    }
    gear_parts
        .iter()
        .filter(|&(_, parts)| parts.len() == 2)
        .map(|(_, parts)| parts[0] * parts[1])
        .sum::<usize>()
}

fn main() {
    let engine_scheme = INPUT.trim().split('\n').collect::<Vec<_>>();
    println!("{}", part_number_sum(&engine_scheme));
    println!("{}", gear_ratio_sum(&engine_scheme));
}
