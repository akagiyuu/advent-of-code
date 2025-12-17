use std::io::stdin;

fn solve(mut grid: Vec<Vec<u8>>) -> u64 {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;

    let mut res = 0;

    loop {
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i as usize][j as usize] != b'@' {
                    continue;
                }

                let mut cur = 0;

                for di in [-1, 0, 1] {
                    for dj in [-1, 0, 1] {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let l = i + di;
                        let k = j + dj;
                        if !(0..n).contains(&l) || !(0..m).contains(&k) {
                            continue;
                        }
                        if grid[l as usize][k as usize] == b'@' {
                            cur += 1;
                        }
                    }
                }

                if cur < 4 {
                    cnt += 1;
                    grid[i as usize][j as usize] = b'.';
                }
            }
        }
        if cnt == 0 {
            break;
        }
        res += cnt;
    }

    res
}

fn main() {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    while let Some(Ok(line)) = stdin().lines().next() {
        grid.push(line.bytes().collect());
    }

    println!("{}", solve(grid));
}
