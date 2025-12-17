use std::io::stdin;

fn solve(grid: &[Vec<u8>]) -> u64 {
    let n = grid.len();
    let m = grid[0].len();

    let mut res = 0;
    let mut line = vec![false; m];
    for j in 0..m {
        if grid[0][j] == b'S' {
            line[j] = true;
            break;
        }
    }
    for i in 1..n {
        for j in 0..m {
            if grid[i][j] == b'.' {
                continue;
            }
            if line[j] {
                res += 1;
                if j >= 1 {
                    line[j - 1] = true;
                }
                if j <= n - 2 {
                    line[j + 1] = true;
                }
                line[j] = false;
            }
        }
    }

    res
}

fn main() {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    while let Some(Ok(line)) = stdin().lines().next() {
        grid.push(line.bytes().collect());
    }
    println!("{}", solve(&grid));
}
