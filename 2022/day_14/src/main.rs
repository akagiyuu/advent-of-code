#![feature(array_windows)]
const INPUT: &str = include_str!("input.txt");

type Point = (usize, usize);

fn shift_paths(paths: &mut [Vec<Point>], sand_drop_point: &mut Point) {
    let rows = paths.iter().flatten().map(|point| point.0).max().unwrap() + 2;

    let min_column = paths
        .iter()
        .flatten()
        .map(|point| point.1)
        .min()
        .unwrap()
        .min(500)
        - rows;
    sand_drop_point.1 -= min_column;
    paths.iter_mut().for_each(|path| {
        path.iter_mut().for_each(|point| {
            point.1 -= min_column;
        })
    })
}

fn generate_grid(paths: &[Vec<Point>], sand_drop_point: &Point) -> Vec<Vec<bool>> {
    let rows = paths.iter().flatten().map(|point| point.0).max().unwrap() + 2;
    let columns = paths
        .iter()
        .flatten()
        .map(|point| point.1)
        .max()
        .unwrap()
        .max(sand_drop_point.1)
        + rows + 1;
    let mut grid = vec![vec![false; columns]; rows];
    for path in paths {
        for &[a, b] in path.as_slice().array_windows::<2>() {
            if a.0 == b.0 {
                let left = a.1.min(b.1);
                let right = a.1.max(b.1);
                for column in left..=right {
                    grid[a.0][column] = true;
                }
                continue;
            }
            if a.1 == b.1 {
                let left = a.0.min(b.0);
                let right = a.0.max(b.0);
                for row in left..=right {
                    grid[row][a.1] = true;
                }
            }
        }
    }
    grid
}
fn print_grid(grid: &[Vec<bool>]) {
    for row in grid {
        for cell in row {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!();
    }
}
fn calculuate_sand_needed_to_reach_bottom(
    mut grid: Vec<Vec<bool>>,
    sand_drop_point: Point,
) -> usize {
    let mut sand_needed = 0;
    loop {
        let mut sand = sand_drop_point;
        loop {
            while sand.0 + 1 < grid.len() && !grid[sand.0 + 1][sand.1] {
                sand.0 += 1;
            }
            if sand.0 + 1 < grid.len() && sand.1 > 0 && !grid[sand.0 + 1][sand.1 - 1] {
                sand = (sand.0 + 1, sand.1 - 1);
                continue;
            }
            if sand.0 + 1 < grid.len()
                && sand.1 + 1 < grid[0].len()
                && !grid[sand.0 + 1][sand.1 + 1]
            {
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            }
            break;
        }
        grid[sand.0][sand.1] = true;
        if sand.0 == grid.len() - 1 {
            break;
        }

        sand_needed += 1;
    }
    sand_needed
}
fn calculuate_sand_needed_to_reach_drop(mut grid: Vec<Vec<bool>>, sand_drop_point: Point) -> usize {
    let mut sand_needed = 0;
    loop {
        let mut sand = sand_drop_point;
        loop {
            while sand.0 + 1 < grid.len() && !grid[sand.0 + 1][sand.1] {
                sand.0 += 1;
            }
            if sand.0 + 1 < grid.len() && sand.1 > 0 && !grid[sand.0 + 1][sand.1 - 1] {
                sand = (sand.0 + 1, sand.1 - 1);
                continue;
            }
            if sand.0 + 1 < grid.len()
                && sand.1 + 1 < grid[0].len()
                && !grid[sand.0 + 1][sand.1 + 1]
            {
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            }
            break;
        }
        grid[sand.0][sand.1] = true;
        sand_needed += 1;

        if sand == sand_drop_point {
            break;
        }
    }
    sand_needed
}

fn main() {
    let mut paths = INPUT
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split_once(',')
                        .map(|(a, b)| (b.parse().unwrap(), a.parse().unwrap()))
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sand_drop_point = (0, 500);
    shift_paths(&mut paths, &mut sand_drop_point);
    let grid = generate_grid(&paths, &sand_drop_point);
    println!(
        "{}",
        calculuate_sand_needed_to_reach_bottom(grid.clone(), sand_drop_point)
    );
    println!(
        "{}",
        calculuate_sand_needed_to_reach_drop(grid.clone(), sand_drop_point)
    );
}
