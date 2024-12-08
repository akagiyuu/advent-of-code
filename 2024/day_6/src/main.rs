use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_start(grid: &[Vec<u8>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'^' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn can_exit(pos: (usize, usize), grid: &[Vec<u8>]) -> bool {
    pos.0 == 0 || pos.0 == grid.len() - 1 || pos.1 == 0 || pos.1 == grid[0].len() - 1
}

fn next_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn apply_dir(pos: (usize, usize), dir: Direction, grid: &[Vec<u8>]) -> (usize, usize) {
    match dir {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
    }
}

fn get_next(
    pos: (usize, usize),
    mut dir: Direction,
    grid: &[Vec<u8>],
) -> ((usize, usize), Direction) {
    loop {
        let new_pos = apply_dir(pos, dir, grid);
        if grid[new_pos.0][new_pos.1] != b'#' {
            return (new_pos, dir);
        }
        dir = next_dir(dir);
    }
}

fn count_visited_position(grid: &[Vec<u8>]) -> usize {
    let mut dir = Direction::Up;
    let mut pos = get_start(grid);
    let mut visited = HashSet::new();
    visited.insert(pos);

    while !can_exit(pos, grid) {
        (pos, dir) = get_next(pos, dir, grid);
        visited.insert(pos);
    }

    visited.len()
}

fn is_loop(grid: &[Vec<u8>]) -> bool {
    let mut dir = Direction::Up;
    let mut pos = get_start(grid);
    let mut visited = HashSet::new();
    visited.insert((pos, dir));

    while !can_exit(pos, grid) {
        (pos, dir) = get_next(pos, dir, grid);
        if !visited.insert((pos, dir)) {
            return true;
        }
    }

    false
}

fn count_valid_config(mut grid: Vec<Vec<u8>>) -> usize {
    let mut dir = Direction::Up;
    let mut pos = get_start(&grid);
    let mut visited = HashSet::new();
    let mut count = 0;
    visited.insert(pos);

    while !can_exit(pos, &grid) {
        (pos, dir) = get_next(pos, dir, &grid);
        visited.insert(pos);
    }

    for (i, j) in visited {
        if grid[i][j] != b'.' {
            continue;
        }
        grid[i][j] = b'#';
        if is_loop(&grid) {
            count += 1;
        }
        grid[i][j] = b'.';
    }

    count
}

fn main() {
    let grid: Vec<_> = INPUT.lines().map(|line| line.as_bytes().to_vec()).collect();

    println!("{}", count_visited_position(&grid));
    println!("{}", count_valid_config(grid));
}
