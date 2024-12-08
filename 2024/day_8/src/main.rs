#![feature(int_roundings)]

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn print_grid(grid: &[Vec<u8>]) {
    for row in grid.iter() {
        println!("{}", String::from_utf8(row.to_vec()).unwrap());
    }
}

fn count_antinode(grid: &[Vec<u8>]) -> usize {
    let row_count = grid.len() as isize;
    let column_count = grid[0].len() as isize;

    let mut antennas = HashMap::<u8, HashSet<(usize, usize)>>::new();
    let mut antinode = HashSet::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if grid[i][j] == b'.' {
                continue;
            }
            let cur_antennas = antennas.entry(cell).or_default();
            for &(prev_i, prev_j) in cur_antennas.iter() {
                let [prev_i, prev_j, i, j] = [prev_i, prev_j, i, j].map(|x| x as isize);
                let dx = i - prev_i;
                let dy = j - prev_j;

                let new_antennas = [(i + dx, j + dy), (prev_i - dx, prev_j - dy)];
                for (new_i, new_j) in new_antennas {
                    if (0..row_count).contains(&new_i) && (0..column_count).contains(&new_j) {
                        let [new_i, new_j] = [new_i, new_j].map(|x| x as usize);
                        antinode.insert((new_i, new_j));
                    }
                }
            }

            cur_antennas.insert((i, j));
        }
    }

    antinode.len()
}

fn count_extend_antinode(grid: &[Vec<u8>]) -> usize {
    let row_count = grid.len() as isize;
    let column_count = grid[0].len() as isize;

    let mut antennas = HashMap::<u8, HashSet<(usize, usize)>>::new();
    let mut antinode = HashSet::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if grid[i][j] == b'.' {
                continue;
            }
            let cur_antennas = antennas.entry(cell).or_default();
            for &(prev_i, prev_j) in cur_antennas.iter() {
                let [prev_i, prev_j, i, j] = [prev_i, prev_j, i, j].map(|x| x as isize);
                let di = i - prev_i;
                let dj = j - prev_j;

                let (i_left, i_right) = if di > 0 {
                    ((-i).div_ceil(di), (row_count - 1 - i).div_floor(di))
                } else {
                    ((row_count - 1 - i).div_ceil(di), (-i).div_floor(di))
                };

                let (j_left, j_right) = if dj > 0 {
                    ((-j).div_ceil(dj), (column_count - 1 - j).div_floor(dj))
                } else {
                    ((column_count - 1 - j).div_ceil(dj), (-j).div_floor(dj))
                };

                let left = i_left.max(j_left);
                let right = i_right.min(j_right);
                for t in left..=right {
                    let cur = ((i + t * di) as usize, (j + t * dj) as usize);
                    antinode.insert(cur);
                }
            }

            cur_antennas.insert((i, j));
        }
    }

    antinode.len()
}

fn main() {
    let grid: Vec<_> = INPUT.lines().map(|line| line.as_bytes().to_vec()).collect();

    println!("{}", count_antinode(&grid));
    println!("{}", count_extend_antinode(&grid));
}
