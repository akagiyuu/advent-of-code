use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn rotate_up(platform: &mut [Vec<u8>]) {
    let row_count = platform.len();
    let column_count = platform[0].len();
    for column in 0..column_count {
        let mut group_start = 0;
        for row in 0..row_count {
            match platform[row][column] {
                b'O' => {
                    platform[row][column] = b'.';
                    platform[group_start][column] = b'O';
                    group_start += 1;
                }
                b'#' => {
                    group_start = row + 1;
                }
                _ => {}
            }
        }
    }
}

fn rotate_down(platform: &mut [Vec<u8>]) {
    let row_count = platform.len();
    let column_count = platform[0].len();
    for column in 0..column_count {
        let mut group_start = row_count - 1;
        for row in (0..row_count).rev() {
            match platform[row][column] {
                b'O' => {
                    platform[row][column] = b'.';
                    platform[group_start][column] = b'O';
                    if group_start == 0 {
                        break;
                    }
                    group_start -= 1;
                }
                b'#' => {
                    if row == 0 {
                        break;
                    }
                    group_start = row - 1;
                }
                _ => {}
            }
        }
    }
}

fn rotate_left(platform: &mut [Vec<u8>]) {
    let row_count = platform.len();
    let column_count = platform[0].len();
    for row in 0..row_count {
        let mut group_start = 0;
        for column in 0..column_count {
            match platform[row][column] {
                b'O' => {
                    platform[row][column] = b'.';
                    platform[row][group_start] = b'O';
                    group_start += 1;
                }
                b'#' => {
                    group_start = column + 1;
                }
                _ => {}
            }
        }
    }
}

fn rotate_right(platform: &mut [Vec<u8>]) {
    let row_count = platform.len();
    let column_count = platform[0].len();
    for row in 0..row_count {
        let mut group_start = column_count - 1;
        for column in (0..column_count).rev() {
            match platform[row][column] {
                b'O' => {
                    platform[row][column] = b'.';
                    platform[row][group_start] = b'O';
                    if group_start == 0 {
                        break;
                    }
                    group_start -= 1;
                }
                b'#' => {
                    if column == 0 {
                        break;
                    }
                    group_start = column - 1;
                }
                _ => {}
            }
        }
    }
}

fn calculate_total_load(platform: &[Vec<u8>]) -> usize {
    let row_count = platform.len();
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&cell| cell == b'O').count() * (row_count - i))
        .sum()
}

fn calculate_total_load_after_n_cycle(platform: Vec<Vec<u8>>, n: usize) -> usize {
    let mut platform = platform;
    let mut cache = HashMap::new();
    cache.insert(platform.clone(), 0);
    let mut i = 1;
    while i <= n {
        rotate_up(&mut platform);
        rotate_left(&mut platform);
        rotate_down(&mut platform);
        rotate_right(&mut platform);
        if let Some(cycle_start) = cache.get(&platform) {
            let cycle_length = i - cycle_start;
            let n = (n - cycle_start) % cycle_length + cycle_start;
            return calculate_total_load(cache.iter().find(|&(_, &i)| i == n).unwrap().0);
        }
        cache.insert(platform.clone(), i);
        i += 1;
    }
    calculate_total_load(&platform)
}

fn calculate_total_load_after_rotate_up(platform: Vec<Vec<u8>>) -> usize {
    let mut platform = platform;
    rotate_up(&mut platform);
    calculate_total_load(&platform)
}

fn main() {
    let platform = INPUT
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    println!("{}", calculate_total_load_after_rotate_up(platform.clone()));
    println!(
        "{}",
        calculate_total_load_after_n_cycle(platform, 1000000000)
    );
}
