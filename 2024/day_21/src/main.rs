// TODO: need heavy redo

use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    usize,
};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Keys {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyUp,
    KeyRight,
    KeyDown,
    KeyLeft,
    Empty,
}

use Keys::*;

fn get_input() -> Vec<(Vec<Keys>, usize)> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            (
                line.chars()
                    .map(|c| match c {
                        '0' => Key0,
                        '1' => Key1,
                        '2' => Key2,
                        '3' => Key3,
                        '4' => Key4,
                        '5' => Key5,
                        '6' => Key6,
                        '7' => Key7,
                        '8' => Key8,
                        '9' => Key9,
                        'A' => KeyA,
                        _ => unreachable!(),
                    })
                    .collect(),
                line[0..3].parse().unwrap(),
            )
        })
        .collect()
}

fn find_shortests(
    start: (usize, usize),
    keypad: &HashMap<(usize, usize), Keys>,
) -> HashMap<Keys, Vec<Vec<Keys>>> {
    let mut paths: HashMap<Keys, Vec<Vec<Keys>>> = HashMap::new();

    let mut to_do: VecDeque<((usize, usize), Vec<Keys>)> = vec![(start, vec![])].into();

    let start_key = keypad.get(&start).unwrap();
    paths.insert(*start_key, vec![]);

    while let Some((coordinate, path)) = to_do.pop_front() {
        let current_key = keypad.get(&coordinate).unwrap();

        let shortest = paths.entry(*current_key).or_default();

        let mut deduped_shortest = shortest.last().unwrap_or(&vec![]).clone();
        deduped_shortest.dedup();
        let mut deduped_path = path.clone();
        deduped_path.dedup();

        if deduped_shortest.is_empty() || deduped_shortest.len() > deduped_path.len() {
            *shortest = vec![path.clone()];
        } else if deduped_shortest.len() == deduped_path.len() {
            (*shortest).push(path.clone());
        } else {
            continue;
        }

        let neighbors = [
            ((coordinate.0.wrapping_sub(1), coordinate.1), KeyUp),
            ((coordinate.0, coordinate.1 + 1), KeyRight),
            ((coordinate.0 + 1, coordinate.1), KeyDown),
            ((coordinate.0, coordinate.1.wrapping_sub(1)), KeyLeft),
        ];

        for (neighbor, direction) in neighbors {
            if let Some(key) = keypad.get(&neighbor) {
                if *key != Empty && key != start_key {
                    let mut next_path = path.clone();
                    next_path.push(direction);
                    to_do.push_back((neighbor, next_path));
                }
            }
        }
    }
    for sub_path in paths.values_mut() {
        for path in sub_path.iter_mut() {
            path.push(KeyA);
        }
    }
    paths
}

fn find_all_shortests(
    keypad: &HashMap<(usize, usize), Keys>,
) -> HashMap<Keys, HashMap<Keys, Vec<Vec<Keys>>>> {
    keypad
        .iter()
        .map(|(coord, key)| (*key, find_shortests(*coord, keypad)))
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let numpad: HashMap<(usize, usize), Keys> = vec![
        ((0, 0), Key7),
        ((0, 1), Key8),
        ((0, 2), Key9),
        ((1, 0), Key4),
        ((1, 1), Key5),
        ((1, 2), Key6),
        ((2, 0), Key1),
        ((2, 1), Key2),
        ((2, 2), Key3),
        ((3, 0), Empty),
        ((3, 1), Key0),
        ((3, 2), KeyA),
    ]
    .into_iter()
    .collect();
    let control: HashMap<(usize, usize), Keys> = vec![
        ((0, 0), Empty),
        ((0, 1), KeyUp),
        ((0, 2), KeyA),
        ((1, 0), KeyLeft),
        ((1, 1), KeyDown),
        ((1, 2), KeyRight),
    ]
    .into_iter()
    .collect();

    let codes = get_input();
    let mut complexities = 0;
    let shortests_paths_numpad = find_all_shortests(&numpad);
    let shortests_paths_control = find_all_shortests(&control);

    for (code, value) in codes {
        let mut previous_key = KeyA;
        let mut len = 0;
        for key in code {
            len += find_recurse(
                (previous_key, key),
                3,
                0,
                &shortests_paths_numpad,
                &shortests_paths_control,
                &mut HashMap::new(),
                &mut HashMap::new(),
            );
            previous_key = key;
        }
        complexities += value * len;
    }

    println!("Total complexities is {}", complexities);

    Ok(())
}

fn find_recurse(
    path: (Keys, Keys),
    max: usize,
    current: usize,
    shortest_numpads: &HashMap<Keys, HashMap<Keys, Vec<Vec<Keys>>>>,
    shortest_controls: &HashMap<Keys, HashMap<Keys, Vec<Vec<Keys>>>>,
    memoization: &mut HashMap<(Keys, Keys), HashMap<usize, usize>>,
    last_at_level: &mut HashMap<usize, Keys>,
) -> usize {
    if current == max {
        1
    } else {
        if let Some(path_cost) = memoization.get(&(path.0, path.1)) {
            if let Some(cost) = path_cost.get(&current) {
                return *cost;
            }
        }

        let next_path = (if current == 0 {
            shortest_numpads
        } else {
            shortest_controls
        })
        .get(&path.0)
        .unwrap()
        .get(&path.1)
        .unwrap();

        let last = *last_at_level.entry(current).or_insert(KeyA);
        let mut next_last = last;
        let mut total = usize::MAX;

        for possible_paths in next_path {
            let mut sub_total = 0;
            let mut previous = last;
            for part in possible_paths {
                let fragment_cost = find_recurse(
                    (previous, *part),
                    max,
                    current + 1,
                    shortest_numpads,
                    shortest_controls,
                    memoization,
                    last_at_level,
                );
                previous = *part;
                sub_total += fragment_cost;
            }
            if sub_total < total {
                total = sub_total;
                next_last = *possible_paths.last().unwrap();
            }
        }

        last_at_level.insert(current, next_last);

        let saved_path = memoization.entry(path).or_default();
        saved_path.insert(current, total);

        total
    }
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let numpad: HashMap<(usize, usize), Keys> = vec![
        ((0, 0), Key7),
        ((0, 1), Key8),
        ((0, 2), Key9),
        ((1, 0), Key4),
        ((1, 1), Key5),
        ((1, 2), Key6),
        ((2, 0), Key1),
        ((2, 1), Key2),
        ((2, 2), Key3),
        ((3, 0), Empty),
        ((3, 1), Key0),
        ((3, 2), KeyA),
    ]
    .into_iter()
    .collect();
    let control: HashMap<(usize, usize), Keys> = vec![
        ((0, 0), Empty),
        ((0, 1), KeyUp),
        ((0, 2), KeyA),
        ((1, 0), KeyLeft),
        ((1, 1), KeyDown),
        ((1, 2), KeyRight),
    ]
    .into_iter()
    .collect();

    let codes = get_input();
    let mut complexities = 0;
    let shortests_paths_numpad = find_all_shortests(&numpad);
    let shortests_paths_control = find_all_shortests(&control);

    for (code, value) in codes {
        let mut previous_key = KeyA;
        let mut len = 0;
        for key in code {
            len += find_recurse(
                (previous_key, key),
                26,
                0,
                &shortests_paths_numpad,
                &shortests_paths_control,
                &mut HashMap::new(),
                &mut HashMap::new(),
            );
            previous_key = key;
        }
        complexities += value * len;
    }

    println!(
        "To save the second historian, the total complexities is {}",
        complexities
    );

    Ok(())
}

fn main() {
    first_star();
    second_star();
}
