const INPUT: &str = include_str!("../input.txt");
const CACHE_FILE: &str = "cache";

use std::{collections::HashMap, fs, iter::repeat_n};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

type Cache = HashMap<State, Vec<Vec<(u8, usize)>>>;

fn get_location(key: u8, keypad_id: u8) -> (usize, usize) {
    match [keypad_id, key] {
        [0, b'7'] => (0, 0),
        [0, b'8'] => (0, 1),
        [0, b'9'] => (0, 2),
        [0, b'4'] => (1, 0),
        [0, b'5'] => (1, 1),
        [0, b'6'] => (1, 2),
        [0, b'1'] => (2, 0),
        [0, b'2'] => (2, 1),
        [0, b'3'] => (2, 2),
        [0, b'.'] => (3, 0),
        [0, b'0'] => (3, 1),
        [0, b'A'] => (3, 2),
        [1, b'.'] => (0, 0),
        [1, b'^'] => (0, 1),
        [1, b'A'] => (0, 2),
        [1, b'<'] => (1, 0),
        [1, b'v'] => (1, 1),
        [1, b'>'] => (1, 2),
        _ => unreachable!(),
    }
}

fn apply((x, y): (usize, usize), dir: u8) -> (usize, usize) {
    match dir {
        b'^' => (x - 1, y),
        b'>' => (x, y + 1),
        b'v' => (x + 1, y),
        b'<' => (x, y - 1),
        _ => unimplemented!(),
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    cur: (usize, usize),
    next: (usize, usize),
    keypad_id: u8,
}

fn get_possible_partial_seq(
    (cx, cy): (usize, usize),
    (nx, ny): (usize, usize),
    keypad_id: u8,
) -> Vec<Vec<(u8, usize)>> {
    if (cx, cy) == (nx, ny) {
        return vec![vec![]];
    }

    let blank = get_location(b'.', keypad_id);

    let vertical_c = if cx < nx { b'v' } else { b'^' };
    let horizontal_c = if cy < ny { b'>' } else { b'<' };

    let dx = cx.abs_diff(nx);
    let dy = cy.abs_diff(ny);

    let required = match (dx, dy) {
        (0, 0) => vec![],
        (_, 0) => vec![vec![(vertical_c, dx)]],
        (0, _) => vec![vec![(horizontal_c, dy)]],
        _ => vec![vec![(vertical_c, dx), (horizontal_c, dy)], vec![
            (horizontal_c, dy),
            (vertical_c, dx),
        ]],
    };

    required
        .into_iter()
        // .par_bridge()
        .filter(|sub| {
            let dirs = sub.iter().flat_map(|&(c, count)| repeat_n(c, count));

            let (mut x, mut y) = (cx, cy);

            for dir in dirs {
                (x, y) = apply((x, y), dir);
                if (x, y) == blank {
                    return false;
                }
            }

            true
        })
        .collect()
}

fn all_cell(keypad_id: u8) -> Vec<(usize, usize)> {
    if keypad_id == 0 {
        (0..4)
            .cartesian_product(0..3)
            .filter(|&x| x != (0, 3))
            .collect_vec()
    } else {
        (0..2)
            .cartesian_product(0..3)
            .filter(|&x| x != (0, 0))
            .collect_vec()
    }
}

fn generate_cache() {
    let mut cache = HashMap::new();

    for keypad_id in [0, 1] {
        for cur in all_cell(keypad_id) {
            for next in all_cell(keypad_id) {
                cache.insert(
                    State {
                        cur,
                        next,
                        keypad_id,
                    },
                    get_possible_partial_seq(cur, next, keypad_id),
                );
            }
        }
    }

    let encoded = bincode::serialize(&cache).unwrap();
    fs::write(CACHE_FILE, encoded).unwrap();
}

fn get_length(code: &[(u8, usize)]) -> usize {
    code.iter().map(|x| x.1).sum()
}

fn print_code(code: &[(u8, usize)]) {
    for &(c, count) in code {
        print!("({}, {}) ", c as char, count);
    }
    println!();
    for &(c, count) in code {
        for _ in 0..count {
            print!("{}", c as char);
        }
    }
    println!()
}

fn get_indirect_control(
    code: &[(u8, usize)],
    keypad_id: u8,
    cache: &Cache,
) -> Vec<Vec<(u8, usize)>> {
    let mut cur = get_location(b'A', keypad_id);
    let mut new_codes = vec![vec![]];

    for &(c, count) in code {
        for _ in 0..count {
            let next = get_location(c, keypad_id);
            let possibles = cache
                .get(&State {
                    cur,
                    next,
                    keypad_id,
                })
                .unwrap();
            let mut next_new_codes = vec![];

            for new_code in new_codes {
                for p in possibles.iter() {
                    let mut tmp = new_code.clone();
                    tmp.extend_from_slice(p);
                    tmp.push((b'A', 1));
                    next_new_codes.push(tmp);
                }
            }

            cur = next;
            new_codes = next_new_codes;
        }
    }

    new_codes
}

fn get_indirect_length(code: &[(u8, usize)], robot_iter: usize, cache: &Cache) -> usize {
    if robot_iter == 0 {
        return get_length(code);
    }

    let mut min_length = usize::MAX;
    let nexts = get_indirect_control(code, 1, cache);
    for next in nexts {
        min_length = min_length.min(get_indirect_length(&next, robot_iter - 1, cache));
    }

    min_length
}

fn get_all_indirect_length(codes: &[Vec<(u8, usize)>], robot_iter: usize, cache: &Cache) -> usize {
    codes
        .iter()
        .map(|code| {
            let value = code
                .iter()
                .map(|(x, _)| x)
                .take_while(|x| x.is_ascii_digit())
                .fold(0, |acc, &x| acc * 10 + (x - b'0') as usize);
            let min_length = get_indirect_control(code, 0, cache)
                .into_iter()
                .map(|code| get_indirect_length(&code, robot_iter, cache))
                .min()
                .unwrap();
            println!("{}", min_length);

            value * min_length
        })
        .sum()
}

fn main() {
    generate_cache();
    let cache: Cache = bincode::deserialize(&fs::read(CACHE_FILE).unwrap()).unwrap();

    let codes = INPUT
        .lines()
        .map(|line| line.bytes().map(|x| (x, 1)).collect_vec())
        .collect_vec();

    println!("{}", get_all_indirect_length(&codes, 2, &cache));
    // println!("{}", get_all_indirect_length(&codes, 25, &cache));
}
