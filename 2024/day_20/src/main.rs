const INPUT: &str = include_str!("../input.txt");

use std::collections::{HashSet, VecDeque};

fn find(c: u8, grid: &[Vec<u8>]) -> Option<(usize, usize)> {
    let m = grid.len();

    grid.iter()
        .flatten()
        .position(|&cell| cell == c)
        .map(|pos| (pos / m, pos % m))
}

fn into_valid((x, y): (isize, isize), grid: &[Vec<u8>]) -> Option<(usize, usize)> {
    let n = grid.len();
    let m = grid[0].len();

    if !(0..n as isize).contains(&x) {
        return None;
    }
    let x = x as usize;

    if !(0..m as isize).contains(&y) {
        return None;
    }
    let y = y as usize;

    if grid[x][y] == b'#' {
        return None;
    }

    Some((x, y))
}

fn generate_distance_map((cx, cy): (usize, usize), grid: &[Vec<u8>]) -> Vec<Vec<usize>> {
    let n = grid.len();
    let m = grid[0].len();

    let mut distance_map = vec![vec![usize::MAX; m]; n];
    distance_map[cx][cy] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((cx, cy));

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nd = distance_map[x][y] + 1;
            let Some((nx, ny)) = into_valid((x as isize + dx, y as isize + dy), grid) else {
                continue;
            };
            if nd < distance_map[nx][ny] {
                distance_map[nx][ny] = nd;
                queue.push_back((nx, ny));
            }
        }
    }

    distance_map
}

fn count_cheat_above_threshold(threshold: usize, cheat_time: usize, grid: &[Vec<u8>]) -> usize {
    let n = grid.len();
    let m = grid[0].len();

    let start = find(b'S', grid).unwrap();
    let end = find(b'E', grid).unwrap();
    let start_dis_map = generate_distance_map(start, grid);
    let end_dis_map = generate_distance_map(end, grid);
    let original_dis = start_dis_map[end.0][end.1];

    let mut count = 0;

    let mut cache = HashSet::<[usize; 4]>::new();
    for x in 0..n {
        for y in 0..m {
            if grid[x][y] == b'#' {
                continue;
            }
            for size_x in 0..=cheat_time {
                for size_y in 0..=cheat_time - size_x {
                    let mut sizes = HashSet::new();
                    if x >= size_x && y >= size_y {
                        sizes.insert((x - size_x, y - size_y));
                    }
                    if x >= size_x && y + size_y < m {
                        sizes.insert((x - size_x, y + size_y));
                    }
                    if x + size_x < n && y >= size_y {
                        sizes.insert((x + size_x, y - size_y));
                    }
                    if x + size_x < n && y + size_y < m {
                        sizes.insert((x + size_x, y + size_y));
                    }

                    for (rx, ry) in sizes {
                        if cache.contains(&[x, y, rx, ry]) || cache.contains(&[rx, ry, x, y]) {
                            continue;
                        }
                        let new_dis = start_dis_map[rx][ry]
                            .saturating_add(end_dis_map[x][y])
                            .min(start_dis_map[x][y].saturating_add(end_dis_map[rx][ry]))
                            .saturating_add(rx.abs_diff(x) + ry.abs_diff(y));
                        if original_dis <= new_dis {
                            continue;
                        }
                        cache.insert([x, y, rx, ry]);
                        if original_dis - new_dis >= threshold {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

fn main() {
    let grid: Vec<_> = INPUT.lines().map(|line| line.as_bytes().to_vec()).collect();

    println!("{}", count_cheat_above_threshold(100, 2, &grid));
    println!("{}", count_cheat_above_threshold(100, 20, &grid));
}
