use std::collections::HashMap;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn sum_mul(input: &str) -> usize {
    let re = Regex::new(r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#).unwrap();

    re.find_iter(input)
        .filter_map(|mul_expr| {
            let raw = mul_expr.as_str();
            let nums = &raw[4..raw.len() - 1];
            let (a, b) = nums.split_once(',')?;

            let a = a.parse::<usize>().ok()?;
            let b = b.parse::<usize>().ok()?;

            Some(a * b)
        })
        .sum()
}

fn sum_mul_with_activation(input: &str) -> usize {
    let re = Regex::new(r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#).unwrap();
    let mul_match: HashMap<_, _> = re
        .find_iter(input)
        .filter_map(|mul_expr| {
            let raw = mul_expr.as_str();
            let nums = &raw[4..raw.len() - 1];
            let (a, b) = nums.split_once(',')?;

            let a = a.parse::<usize>().ok()?;
            let b = b.parse::<usize>().ok()?;

            Some((mul_expr.end() - 1, a * b))
        })
        .collect();

    let mut is_activate = true;
    let input = input.as_bytes();
    let length = input.len();
    let mut res = 0;

    for i in 3..length {
        if input[i] != b')' {
            continue;
        }
        if &input[i - 3..=i] == b"do()" {
            is_activate = true;
            continue;
        }
        if &input[i - 6..=i] == b"don't()" {
            is_activate = false;
            continue;
        }

        if !is_activate {
            continue;
        }

        if let Some(value) = mul_match.get(&i) {
            res += value
        }
    }

    res
}

fn main() {
    println!("{}", sum_mul(INPUT));
    println!("{}", sum_mul_with_activation(INPUT));
}
