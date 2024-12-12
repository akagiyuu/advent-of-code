use std::collections::{BTreeSet, HashSet};

const INPUT: &str = include_str!("../input.txt");

const MAIN_DIRECTION: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn calculate_total_price(garden: &[Vec<u8>]) -> usize {
    let row_count = garden.len() as isize;
    let column_count = garden[0].len() as isize;

    let mut total_price = 0;
    let mut is_visited = vec![vec![false; column_count as usize]; row_count as usize];
    let mut stack = BTreeSet::new();

    for (i, row) in garden.iter().enumerate() {
        for (j, &group) in row.iter().enumerate() {
            if is_visited[i][j] {
                continue;
            };
            let mut perimeter = 0;
            let mut area = 0;

            stack.clear();
            stack.insert((i, j));
            while let Some((i, j)) = stack.pop_first() {
                is_visited[i][j] = true;
                area += 1;
                perimeter += 4;

                for (di, dj) in MAIN_DIRECTION {
                    let next_i = i as isize + di;
                    let next_j = j as isize + dj;
                    if !(0..row_count).contains(&next_i) || !(0..column_count).contains(&next_j) {
                        continue;
                    }
                    let [next_i, next_j] = [next_i, next_j].map(|x| x as usize);
                    if garden[next_i][next_j] == group {
                        perimeter -= 1;

                        if !is_visited[next_i][next_j] {
                            stack.insert((next_i, next_j));
                        }
                    }
                }
            }

            total_price += perimeter * area;
        }
    }

    total_price
}

fn calculate_total_price_with_discount(garden: &[Vec<u8>]) -> u64 {
    let row_count = garden.len() as isize;
    let column_count = garden[0].len() as isize;

    let mut total_price = 0;
    let mut is_visited = vec![vec![false; column_count as usize]; row_count as usize];
    let mut stack = BTreeSet::new();
    let mut vertices = HashSet::new();

    for (i, row) in garden.iter().enumerate() {
        for (j, &group) in row.iter().enumerate() {
            if is_visited[i][j] {
                continue;
            };
            eprintln!("DEBUGPRINT[10]: main.rs:64: group={:#?}", group as char);
            let mut area = 0;

            vertices.clear();
            stack.clear();
            stack.insert((i, j));
            while let Some((i, j)) = stack.pop_first() {
                is_visited[i][j] = true;
                area += 1;

                for dvi in [0, 1] {
                    for dvj in [0, 1] {
                        vertices.insert((i + dvi, j + dvj));
                    }
                }
                for (di, dj) in MAIN_DIRECTION {
                    let next_i = i as isize + di;
                    let next_j = j as isize + dj;
                    if !(0..row_count).contains(&next_i) || !(0..column_count).contains(&next_j) {
                        continue;
                    }
                    let [next_i, next_j] = [next_i, next_j].map(|x| x as usize);
                    if garden[next_i][next_j] == group && !is_visited[next_i][next_j] {
                        stack.insert((next_i, next_j));
                    }
                }
            }

            let mut corner_count = 0;
            for &(i, j) in &vertices {
                let mut neighbor = 0;

                for di in [-1, 0, 1] {
                    for dj in [-1, 0, 1] {
                        if di == 0 && dj == 0 {
                            continue;
                        }

                        let next_i = i as isize + di;
                        let next_j = j as isize + dj;
                        if !(0..=row_count).contains(&next_i)
                            || !(0..=column_count).contains(&next_j)
                        {
                            continue;
                        }
                        let [next_i, next_j] = [next_i, next_j].map(|x| x as usize);
                        if vertices.contains(&(next_i, next_j)) {
                            if i == 1 && j == 1 {
                                println!("{}, {}", next_i, next_j);
                            }
                            neighbor += 1;
                        }
                    }
                }
                //
                // println!("{}, {} -> {}", i, j, neighbor);
                match neighbor {
                    3 | 4 | 7 => corner_count += 1,
                    _ => {}
                }
            }

            total_price += corner_count * area;
        }
    }

    total_price
}

fn main() {
    let garden: Vec<_> = INPUT.lines().map(|line| line.as_bytes().to_vec()).collect();

    println!("{}", calculate_total_price(&garden));
    println!("{}", calculate_total_price_with_discount(&garden));
}
