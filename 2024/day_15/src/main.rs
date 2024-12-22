#![feature(iter_array_chunks)]

const INPUT: &str = include_str!("../input.txt");
const DIRS: [u8; 4] = [b'^', b'>', b'v', b'<'];

fn into_dir(raw: u8) -> u8 {
    DIRS.into_iter().position(|x| x == raw).unwrap() as u8
}

fn apply((x, y): (usize, usize), dir: u8) -> (usize, usize) {
    match dir {
        0 => (x - 1, y),
        1 => (x, y + 1),
        2 => (x + 1, y),
        3 => (x, y - 1),
        _ => unreachable!(),
    }
}

fn is_moveable((x, y): (usize, usize), dir: u8, grid: &[Vec<u8>]) -> bool {
    match grid[x][y] {
        b'#' => false,
        b'.' => true,
        b'@' | b'O' => is_moveable(apply((x, y), dir), dir, grid),
        b'[' if dir % 2 == 1 => is_moveable(apply((x, y), dir), dir, grid),
        b'[' => {
            is_moveable(apply((x, y), dir), dir, grid)
                && is_moveable(apply((x, y + 1), dir), dir, grid)
        }
        b']' if dir % 2 == 1 => is_moveable(apply((x, y), dir), dir, grid),
        b']' => is_moveable((x, y - 1), dir, grid),
        _ => unreachable!(),
    }
}

fn move_obj((x, y): (usize, usize), dir: u8, grid: &mut [Vec<u8>]) {
    match grid[x][y] {
        b'@' | b'O' => {
            let (nx, ny) = apply((x, y), dir);
            move_obj((nx, ny), dir, grid);
            grid[nx][ny] = grid[x][y];
            grid[x][y] = b'.';
        }
        b'[' if dir % 2 == 1 => {
            let (nx, ny) = apply((x, y), dir);
            move_obj((nx, ny), dir, grid);
            grid[nx][ny] = grid[x][y];
            grid[x][y] = b'.';
        }
        b']' if dir % 2 == 1 => {
            let (nx, ny) = apply((x, y), dir);
            move_obj((nx, ny), dir, grid);
            grid[nx][ny] = grid[x][y];
            grid[x][y] = b'.';
        }
        b'[' => {
            let (rx, ry) = (x, y + 1);
            let (nx, ny) = apply((x, y), dir);
            let (nrx, nry) = apply((rx, ry), dir);
            move_obj((nx, ny), dir, grid);
            move_obj((nrx, nry), dir, grid);
            grid[nx][ny] = grid[x][y];
            grid[x][y] = b'.';

            grid[nrx][nry] = grid[rx][ry];
            grid[rx][ry] = b'.';
        }
        b']' => move_obj((x, y - 1), dir, grid),
        _ => {}
    }
}

fn move_robot((x, y): (usize, usize), dir: u8, grid: &mut [Vec<u8>]) -> (usize, usize) {
    if !is_moveable((x, y), dir, grid) {
        return (x, y);
    }
    move_obj((x, y), dir, grid);
    apply((x, y), dir)
}

fn find_robot(grid: &[Vec<u8>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'@' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn gps_sum(mut grid: Vec<Vec<u8>>, dirs: impl Iterator<Item = u8>) -> usize {
    let robot = find_robot(&grid);
    dirs.fold(robot, |robot, dir| move_robot(robot, dir, &mut grid));

    grid.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (j, &cell)| {
            acc + if cell == b'O' || cell == b'[' {
                100 * i + j
            } else {
                0
            }
        })
    })
}

fn map_grid(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|x| match x {
                    b'#' => [b'#'; 2],
                    b'.' => [b'.'; 2],
                    b'O' => [b'[', b']'],
                    b'@' => [b'@', b'.'],
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn main() {
    let (grid, dirs) = INPUT.split_once("\n\n").unwrap();
    let grid: Vec<Vec<u8>> = grid
        .lines()
        .map(|line| line.trim().bytes().collect())
        .collect();
    let dirs = dirs.lines().flat_map(|line| line.bytes().map(into_dir));
    println!("{}", gps_sum(grid.clone(), dirs.clone()));
    println!("{}", gps_sum(map_grid(grid), dirs));
}
