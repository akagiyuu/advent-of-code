const INPUT: &str = include_str!("../input.txt");

use std::collections::{HashMap, VecDeque};

fn print_grid(grid: &[Vec<u8>]) {
    for row in grid.iter() {
        println!("{}", String::from_utf8(row.to_vec()).unwrap());
    }
}

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

fn unlock(
    queue: &mut VecDeque<(usize, usize)>,
    mut distance_map: Vec<Vec<usize>>,
    grid: &[Vec<u8>],
) -> Vec<Vec<usize>> {
    while let Some((x, y)) = queue.pop_front() {
        let mut min_neighbor = usize::MAX;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let Some((nx, ny)) = into_valid((x as isize + dx, y as isize + dy), grid) else {
                continue;
            };
            min_neighbor = min_neighbor.min(distance_map[nx][ny]);
        }

        if min_neighbor == usize::MAX || min_neighbor + 1 >= distance_map[x][y] {
            continue;
        }

        distance_map[x][y] = min_neighbor + 1;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let Some((nx, ny)) = into_valid((x as isize + dx, y as isize + dy), grid) else {
                continue;
            };
            queue.push_back((nx, ny));
        }
    }

    distance_map
}

fn count_cheat_above_threshold(threshold: usize, mut grid: Vec<Vec<u8>>) -> usize {
    let n = grid.len();
    let m = grid[0].len();
    let mut test = HashMap::<usize, usize>::new();

    let start = find(b'S', &grid).unwrap();
    let end = find(b'E', &grid).unwrap();
    let distance_map = generate_distance_map(end, &grid);
    let original_dis = distance_map[start.0][start.1];
    let mut queue = VecDeque::new();
    let mut count = 0;

    for x in 0..n {
        for y in 0..m {
            for (dx, dy) in [(1, 0), (0, 1)] {
                let Some((nx, ny)) = into_valid((x as isize + dx, y as isize + dy), &grid) else {
                    continue;
                };
                if grid[x][y] != b'#' && grid[nx][ny] != b'#' {
                    continue;
                }

                let tmp = (grid[x][y], grid[nx][ny]);
                grid[x][y] = b'.';
                grid[nx][ny] = b'.';

                queue.push_back((x, y));
                queue.push_back((nx, ny));

                let new_dis_map = unlock(&mut queue, distance_map.clone(), &grid);
                let new_dis = new_dis_map[start.0][start.1];
                *test.entry(original_dis - new_dis).or_default() += 1;
                if original_dis - new_dis >= threshold {
                    count += 1;
                }

                (grid[x][y], grid[nx][ny]) = tmp;
            }
        }
    }

    count
}

fn main() {
    let grid: Vec<_> = INPUT.lines().map(|line| line.as_bytes().to_vec()).collect();

    println!("{}", count_cheat_above_threshold(100, grid));
}
