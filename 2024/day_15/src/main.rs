const INPUT: &str = include_str!("../input.txt");

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Direction::Up,
            b'>' => Direction::Right,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

fn step(
    (ri, rj): (usize, usize),
    dir: Direction,
    mut grid: Vec<Vec<u8>>,
) -> (Vec<Vec<u8>>, (usize, usize)) {
    match dir {
        Direction::Up => {
            let mut i = ri - 1;
            while grid[i][rj] == b'O' {
                i -= 1;
            }
            if grid[i][rj] == b'#' {
                return (grid, (ri, rj));
            }
            grid[i][rj] = grid[ri - 1][rj];
            grid[ri - 1][rj] = grid[ri][rj];
            grid[ri][rj] = b'.';

            (grid, (ri - 1, rj))
        }
        Direction::Right => {
            let mut j = rj + 1;
            while grid[ri][j] == b'O' {
                j += 1;
            }
            if grid[ri][j] == b'#' {
                return (grid, (ri, rj));
            }
            grid[ri][j] = grid[ri][rj + 1];
            grid[ri][rj + 1] = grid[ri][rj];
            grid[ri][rj] = b'.';

            (grid, (ri, rj + 1))
        }
        Direction::Down => {
            let mut i = ri + 1;
            while grid[i][rj] == b'O' {
                i += 1;
            }
            if grid[i][rj] == b'#' {
                return (grid, (ri, rj));
            }
            grid[i][rj] = grid[ri + 1][rj];
            grid[ri + 1][rj] = grid[ri][rj];
            grid[ri][rj] = b'.';

            (grid, (ri + 1, rj))
        }
        Direction::Left => {
            let mut j = rj - 1;
            while grid[ri][j] == b'O' {
                j -= 1;
            }
            if grid[ri][j] == b'#' {
                return (grid, (ri, rj));
            }
            grid[ri][j] = grid[ri][rj - 1];
            grid[ri][rj - 1] = grid[ri][rj];
            grid[ri][rj] = b'.';

            (grid, (ri, rj - 1))
        }
    }
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

fn print_grid(grid: &[Vec<u8>]) {
    for row in grid.iter() {
        println!("{}", String::from_utf8(row.to_vec()).unwrap());
    }
}

fn gps_sum(grid: Vec<Vec<u8>>, dirs: impl Iterator<Item = Direction>) -> usize {
    let robot = find_robot(&grid);
    let (grid, _) = dirs.fold((grid, robot), |(grid, robot), dir| {
        // print_grid(&grid);
        // println!();
        step(robot, dir, grid)
    });
    // print_grid(&grid);
    // println!();

    grid.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (j, &cell)| {
            acc + if cell == b'O' { 100 * i + j } else { 0 }
        })
    })
}

fn main() {
    let (grid, dirs) = INPUT.split_once("\n\n").unwrap();
    let grid = grid
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect();
    let dirs = dirs
        .lines()
        .flat_map(|line| line.as_bytes())
        .map(|dir| Direction::from(*dir));
    println!("{}", gps_sum(grid, dirs));
}
