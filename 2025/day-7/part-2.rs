use std::{io::stdin, vec};

fn solve(mut grid: Vec<Vec<u8>>) -> u64 {
    let n = grid.len();
    let m = grid[0].len();

    let mut line = vec![vec![]; m];
    let mut node = 0;
    let mut adj = vec![vec![]];
    let mut vertex = vec![];

    for j in 0..m {
        if grid[0][j] == b'S' {
            line[j].push(node);
            vertex.push((0, j));
            break;
        }
    }
    node += 1;

    for i in 1..n {
        for j in 0..m {
            if grid[i][j] == b'.' || line[j].is_empty() {
                continue;
            }
            for &prev in &line[j] {
                adj[prev].push(node);
            }
            if j >= 1 {
                line[j - 1].push(node);
            }
            if j <= m - 2 {
                line[j + 1].push(node);
            }
            line[j].clear();
            adj.push(vec![]);
            vertex.push((i, j));
            node += 1;
        }
    }

    let n = adj.len();
    let mut dp = vec![0u64; n];
    for u in (0..n).rev() {
        let (_, j) = vertex[u];
        if j >= 1 {
            dp[u] += 1;
        }
        if j <= m - 2 {
            dp[u] += 1;
        }

        if adj[u].is_empty() {
            continue;
        }

        for &v in &adj[u] {
            dp[u] += dp[v];
            dp[u] -= 1;
        }
    }

    dp[0] - 1
}

fn main() {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    while let Some(Ok(line)) = stdin().lines().next() {
        grid.push(line.bytes().collect());
    }
    println!("{}", solve(grid));
}
