use std::collections::HashSet;

use helpers::{Coordinate, Direction};
const INPUT: &str = include_str!("input.txt");

fn count_reachable_plots(garden: &[Vec<u8>], start: Coordinate, max_steps: usize) -> usize {
    let row_count = garden.len();
    let column_count = garden[0].len();

    let mut current_reachable_plots: HashSet<Coordinate> = HashSet::from([start]);
    for _ in 1..=max_steps {
        let mut next_reachable_plots = HashSet::new();
        for current in current_reachable_plots.iter() {
            for direction in Direction::all() {
                let Some(next) = direction.apply(*current, 1, 0..row_count, 0..column_count) else {
                    continue;
                };
                if garden[next.0][next.1] == b'#' {
                    continue;
                }
                next_reachable_plots.insert(next);
            }
        }
        current_reachable_plots = next_reachable_plots;
    }
    current_reachable_plots.len()
}

fn count_reachable_plot_with_infinite_garden(
    garden: &[Vec<u8>],
    start: (i64, i64),
    max_steps: usize,
) -> usize {
    let row_count = garden.len();
    let column_count = garden[0].len();

    let mut current_reachable_plots: HashSet<(i64, i64)> = HashSet::from([start]);
    for _ in 1..=max_steps {
        let mut next_reachable_plots = HashSet::new();
        for current in current_reachable_plots.iter() {
            for direction in Direction::all() {
                let next = match direction {
                    Direction::Right => (current.0, current.1 + 1),
                    Direction::Down => (current.0 + 1, current.1),
                    Direction::Left => (current.0, current.1 - 1),
                    Direction::Up => (current.0 - 1, current.1),
                };
                let actual_next = (
                    ((next.0 % row_count as i64 + row_count as i64) as usize) % row_count,
                    ((next.1 % column_count as i64 + column_count as i64) as usize) % column_count,
                );
                if garden[actual_next.0][actual_next.1] == b'#' {
                    continue;
                }
                next_reachable_plots.insert(next);
            }
        }
        current_reachable_plots = next_reachable_plots;
    }
    current_reachable_plots.len()
}

fn main() {
    let mut garden = INPUT
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut start = (0, 0);
    let row_count = garden.len();
    let column_count = garden[0].len();
    for i in 0..row_count {
        for j in 0..column_count {
            if garden[i][j] == b'S' {
                start = (i as i64, j as i64);
                garden[i][j] = b'.';
            }
        }
    }
    println!(
        "{}",
        count_reachable_plots(&garden, (start.0 as usize, start.1 as usize), 64)
    );

    // After row_count step c additional starts are reached so we have number of starts after n * row_count is c * n so
    // plot_count(n * row_count + d) = plot_count((n - 1) * row_count + d) + (n-1) * c * e
    // so plot_count is quadratic function on n
    println!("{}", row_count);
    println!("{}", 26501365 % row_count);
    println!("{}", 26501365 / column_count);
    for i in 0..=2 {
        let plot_count = count_reachable_plot_with_infinite_garden(
            &garden,
            start,
            26501365 % row_count + row_count * i,
        );
        println!("{}", plot_count);
    }
}
