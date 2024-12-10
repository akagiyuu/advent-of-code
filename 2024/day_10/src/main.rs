use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn calculate_sum_score(grid: &[Vec<u8>]) -> usize {
    let mut res = 0;

    let mut stack = Vec::new();
    let mut reachable = HashSet::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != 0 {
                continue;
            }
            stack.clear();
            reachable.clear();
            stack.push((i, j));

            while let Some((i, j)) = stack.pop() {
                if grid[i][j] == 9 {
                    reachable.insert((i, j));
                    continue;
                }
                if i > 0 && grid[i - 1][j] == grid[i][j] + 1 {
                    stack.push((i - 1, j));
                }
                if i < grid.len() - 1 && grid[i + 1][j] == grid[i][j] + 1 {
                    stack.push((i + 1, j));
                }

                if j > 0 && grid[i][j - 1] == grid[i][j] + 1 {
                    stack.push((i, j - 1));
                }
                if j < grid[0].len() - 1 && grid[i][j + 1] == grid[i][j] + 1 {
                    stack.push((i, j + 1));
                }
            }

            res += reachable.len();
        }
    }

    res
}

fn calculate_sum_rating(grid: &[Vec<u8>]) -> usize {
    let mut res = 0;

    let mut stack = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != 0 {
                continue;
            }
            stack.clear();
            stack.push((i, j));

            while let Some((i, j)) = stack.pop() {
                if grid[i][j] == 9 {
                    res += 1;
                    continue;
                }
                if i > 0 && grid[i - 1][j] == grid[i][j] + 1 {
                    stack.push((i - 1, j));
                }
                if i < grid.len() - 1 && grid[i + 1][j] == grid[i][j] + 1 {
                    stack.push((i + 1, j));
                }

                if j > 0 && grid[i][j - 1] == grid[i][j] + 1 {
                    stack.push((i, j - 1));
                }
                if j < grid[0].len() - 1 && grid[i][j + 1] == grid[i][j] + 1 {
                    stack.push((i, j + 1));
                }
            }
        }
    }

    res
}

fn main() {
    let grid: Vec<_> = INPUT
        .lines()
        .map(|line| line.bytes().map(|x| x - b'0').collect())
        .collect();
    println!("{}", calculate_sum_score(&grid));
    println!("{}", calculate_sum_rating(&grid));
}
