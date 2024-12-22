const INPUT: &str = include_str!("../input.txt");

use std::collections::VecDeque;

use disjoint_hash_set::DisjointHashSet;

const MAIN_DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn into_valid((x, y): (isize, isize), grid: &[Vec<u8>]) -> Option<(usize, usize)> {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;

    if !(0..n).contains(&x) || !(0..m).contains(&y) {
        return None;
    }
    let (x, y) = (x as usize, y as usize);

    if grid[x][y] == b'#' {
        None
    } else {
        Some((x, y))
    }
}

fn get_shortest_path(mut grid: Vec<Vec<u8>>) -> usize {
    let n = grid.len();
    let m = grid[0].len();

    let mut distances = vec![vec![usize::MAX; m]; n];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    distances[0][0] = 0;

    while let Some((x, y)) = queue.pop_front() {
        grid[x][y] = b'#';

        for (nx, ny) in MAIN_DIRECTIONS
            .into_iter()
            .flat_map(|(dx, dy)| into_valid((x as isize + dx, y as isize + dy), &grid))
        {
            let cur_dis = distances[x][y] + 1;
            if cur_dis < distances[nx][ny] {
                distances[nx][ny] = cur_dis;
                queue.push_back((nx, ny));
            }
        }
    }

    distances[n - 1][m - 1]
}

fn build_dsu(grid: &[Vec<u8>]) -> DisjointHashSet<(usize, usize)> {
    let n = grid.len();
    let m = grid[0].len();

    let mut dsu = DisjointHashSet::new();

    for x in 0..n {
        for y in 0..m {
            if grid[x][y] == b'#' {
                continue;
            }
            for (nx, ny) in MAIN_DIRECTIONS
                .into_iter()
                .flat_map(|(dx, dy)| into_valid((x as isize + dx, y as isize + dy), grid))
            {
                dsu.link((x, y), (nx, ny));
            }
        }
    }

    dsu
}

fn unlock((x, y): (usize, usize), dsu: &mut DisjointHashSet<(usize, usize)>, grid: &mut [Vec<u8>]) {
    grid[x][y] = b'.';
    for (nx, ny) in MAIN_DIRECTIONS
        .into_iter()
        .flat_map(|(dx, dy)| into_valid((x as isize + dx, y as isize + dy), grid))
    {
        dsu.link((x, y), (nx, ny));
    }
}

fn get_critical_lock(
    rev_obstacle: impl Iterator<Item = (usize, usize)>,
    mut grid: Vec<Vec<u8>>,
) -> (usize, usize) {
    let n = grid.len();
    let m = grid[0].len();

    let mut dsu = build_dsu(&grid);

    for (x, y) in rev_obstacle.into_iter() {
        unlock((x, y), &mut dsu, &mut grid);
        if dsu.is_linked((0, 0), (n - 1, m - 1)) {
            return (y, x);
        }
    }

    unreachable!()
}

fn main() {
    let mut grid = vec![vec![b'.'; 71]; 71];
    let obstacles = INPUT.lines().map(|raw| {
        let (y, x) = raw.split_once(',').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        (x, y)
    });

    for (x, y) in obstacles.clone().take(1024) {
        grid[x][y] = b'#';
    }

    println!("{}", get_shortest_path(grid.clone()));

    for (x, y) in obstacles.clone() {
        grid[x][y] = b'#';
    }

    println!("{:?}", get_critical_lock(obstacles.clone().rev(), grid))
}
