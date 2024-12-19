const INPUT: &str = include_str!("../input.txt");

use std::collections::VecDeque;

fn get_shortest_path(mut grid: Vec<Vec<u8>>) -> usize {
    let row_count = grid.len();
    let column_count = grid[0].len();

    let mut distances = vec![vec![usize::MAX; column_count]; row_count];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    distances[0][0] = 0;

    let parse = move |raw: (isize, isize)| -> Option<(usize, usize)> {
        if (0..row_count as isize).contains(&raw.0) && (0..column_count as isize).contains(&raw.1) {
            Some((raw.0 as usize, raw.1 as usize))
        } else {
            None
        }
    };

    while let Some((x, y)) = queue.pop_front() {
        grid[x][y] = b'#';

        for (nx, ny) in [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
            .flat_map(&parse)
        {
            if grid[nx][ny] != b'.' {
                continue;
            }
            let cur_dis = distances[x][y] + 1;
            if cur_dis < distances[nx][ny] {
                distances[nx][ny] = cur_dis;
                queue.push_back((nx, ny));
            }
        }
    }

    distances[row_count - 1][column_count - 1]
}

fn main() {
    let mut grid = vec![vec![b'.'; 71]; 71];

    for raw in INPUT.lines().take(1024) {
        let (x, y) = raw.split_once(',').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();

        grid[x][y] = b'#';
    }
    // let grid = vec![
    //     (b"...#...").to_vec(),
    //     (b"..#..#.").to_vec(),
    //     (b"....#..").to_vec(),
    //     (b"...#..#").to_vec(),
    //     (b"..#..#.").to_vec(),
    //     (b".#..#..").to_vec(),
    //     (b"#.#....").to_vec(),
    // ];

    println!("{}", get_shortest_path(grid));
}
